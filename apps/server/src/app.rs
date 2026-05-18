use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::{HeaderValue, Method},
    routing::{get, post},
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use uuid::Uuid;

use crate::{
    ai::{ZeroGComputeClient, build_proof_record},
    chain::ChainClient,
    config::Config,
    db,
    error::ApiError,
    models::{
        Capsule, CapsuleRun, ChainCapsuleRecord, ChainPublishRequest, ChainPublishResponse,
        ChainStatus, CreateCapsuleRequest, CreateRunRequest, PublishCapsuleRequest, RunStatus,
    },
};

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub pool: sqlx::SqlitePool,
    pub ai: ZeroGComputeClient,
    pub chain: ChainClient,
}

pub async fn build_state(config: Config) -> anyhow::Result<Arc<AppState>> {
    let pool = db::connect(&config.database_url).await?;
    let ai = ZeroGComputeClient::new(
        config.zero_g_router_base_url.clone(),
        config.zero_g_router_model.clone(),
        config.zero_g_router_api_key.clone(),
    );
    let chain = ChainClient::new(
        config.zero_g_chain_rpc_url.clone(),
        config.zero_g_chain_id,
        config.skillcapsule_registry_contract.clone(),
        config.skillcapsule_private_key.clone(),
        config.zero_g_chain_explorer_tx_base.clone(),
    );
    Ok(Arc::new(AppState {
        config,
        pool,
        ai,
        chain,
    }))
}

pub fn router(state: Arc<AppState>) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(tower_http::cors::Any)
        .allow_origin(
            state
                .config
                .web_origin
                .parse::<HeaderValue>()
                .unwrap_or_else(|_| HeaderValue::from_static("http://127.0.0.1:5179")),
        );

    Router::new()
        .route("/health", get(health))
        .route("/api/chain/status", get(chain_status))
        .route("/api/capsules", get(list_capsules).post(create_capsule))
        .route("/api/capsules/{id}", get(get_capsule))
        .route("/api/capsules/{id}/chain", get(get_capsule_chain))
        .route("/api/capsules/{id}/publish", post(publish_capsule))
        .route(
            "/api/capsules/{id}/publish/onchain",
            post(publish_capsule_onchain),
        )
        .route("/api/capsules/{id}/run", post(run_capsule))
        .route("/api/runs/{id}", get(get_run))
        .with_state(state)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
}

async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "status": "ok" }))
}

async fn chain_status(State(state): State<Arc<AppState>>) -> Json<ChainStatus> {
    Json(state.chain.status())
}

async fn list_capsules(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Capsule>>, ApiError> {
    Ok(Json(db::list_capsules(&state.pool).await?))
}

async fn create_capsule(
    State(state): State<Arc<AppState>>,
    Json(input): Json<CreateCapsuleRequest>,
) -> Result<Json<Capsule>, ApiError> {
    validate_capsule_request(&input)?;
    Ok(Json(db::create_capsule(&state.pool, input).await?))
}

async fn get_capsule(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Capsule>, ApiError> {
    Ok(Json(db::get_capsule(&state.pool, id).await.map_err(
        |_| ApiError::not_found("Capsule was not found."),
    )?))
}

async fn get_capsule_chain(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ChainCapsuleRecord>, ApiError> {
    if !state.chain.read_enabled() {
        return Err(ApiError::bad_request(
            "ZERO_G_CHAIN_RPC_URL and SKILLCAPSULE_REGISTRY_CONTRACT_ADDRESS are required.",
        ));
    }
    let capsule = db::get_capsule(&state.pool, id)
        .await
        .map_err(|_| ApiError::not_found("Capsule was not found."))?;
    Ok(Json(state.chain.get_capsule(&capsule).await?))
}

async fn publish_capsule(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(input): Json<PublishCapsuleRequest>,
) -> Result<Json<Capsule>, ApiError> {
    if input.registry_contract.trim().is_empty() || input.tx_hash.trim().is_empty() {
        return Err(ApiError::bad_request(
            "registryContract and txHash are required.",
        ));
    }
    let capsule = db::publish_capsule(
        &state.pool,
        id,
        input.registry_contract,
        input.tx_hash,
        input.explorer_url,
        input.agent_token_id,
        input.version,
    )
    .await?;
    Ok(Json(capsule))
}

async fn publish_capsule_onchain(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(input): Json<ChainPublishRequest>,
) -> Result<Json<ChainPublishResponse>, ApiError> {
    if !state.chain.write_enabled() {
        return Err(ApiError::bad_request(
            "ZERO_G_CHAIN_RPC_URL, SKILLCAPSULE_REGISTRY_CONTRACT_ADDRESS, and SKILLCAPSULE_PRIVATE_KEY are required.",
        ));
    }
    let capsule = db::get_capsule(&state.pool, id)
        .await
        .map_err(|_| ApiError::not_found("Capsule was not found."))?;
    let version = input.version.unwrap_or(capsule.version);
    if version < 1 {
        return Err(ApiError::bad_request("version must be greater than zero."));
    }
    let agent_token_id = input
        .agent_token_id
        .as_deref()
        .or(capsule.manifest.agent_token_id.as_deref());
    let agent_token_id = ChainClient::parse_agent_token_id(agent_token_id)?;
    let chain = state
        .chain
        .publish_capsule(
            &capsule,
            input.registry_contract.as_deref(),
            version as u64,
            agent_token_id,
        )
        .await?;
    let updated = db::publish_capsule(
        &state.pool,
        id,
        chain.registry_contract.clone(),
        chain.tx_hash.clone(),
        chain.explorer_url.clone(),
        Some(agent_token_id.to_string()),
        Some(version),
    )
    .await?;

    Ok(Json(ChainPublishResponse {
        capsule: updated,
        chain,
    }))
}

async fn run_capsule(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(input): Json<CreateRunRequest>,
) -> Result<Json<CapsuleRun>, ApiError> {
    if input.task_input.trim().len() < 8 {
        return Err(ApiError::bad_request(
            "taskInput must be at least 8 characters.",
        ));
    }
    let capsule = db::get_capsule(&state.pool, id)
        .await
        .map_err(|_| ApiError::not_found("Capsule was not found."))?;
    let run = db::insert_run(&state.pool, &capsule, &input.task_input).await?;
    let state_for_task = state.clone();
    let run_id = run.id;
    let task_input = input.task_input.clone();

    tokio::spawn(async move {
        if let Err(error) = execute_run(state_for_task, capsule, run_id, task_input).await {
            tracing::error!(%error, "capsule run failed");
        }
    });

    Ok(Json(run))
}

async fn get_run(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<CapsuleRun>, ApiError> {
    Ok(Json(
        db::get_run(&state.pool, id)
            .await
            .map_err(|_| ApiError::not_found("Run was not found."))?,
    ))
}

async fn execute_run(
    state: Arc<AppState>,
    capsule: Capsule,
    run_id: Uuid,
    task_input: String,
) -> anyhow::Result<()> {
    db::update_run_status(
        &state.pool,
        run_id,
        RunStatus::Running,
        None,
        None,
        Vec::new(),
        None,
    )
    .await?;

    match state.ai.run_capsule(&capsule.manifest, &task_input).await {
        Ok(output) => {
            let proof = build_proof_record(
                &capsule.manifest,
                &capsule.manifest_root,
                capsule.registry_contract.clone(),
                capsule.publish_tx_hash.clone(),
                capsule.explorer_url.clone(),
                None,
                state.ai.model(),
                &task_input,
                &output.output_text,
            );
            let mut warnings = output.warnings;
            warnings.push(format!("proofHash={}", proof.proof_hash));
            db::update_run_status(
                &state.pool,
                run_id,
                RunStatus::Completed,
                Some(output.output_json),
                Some(output.output_text),
                warnings,
                None,
            )
            .await?;
        }
        Err(error) => {
            db::update_run_status(
                &state.pool,
                run_id,
                RunStatus::Failed,
                None,
                None,
                vec![error.to_string()],
                None,
            )
            .await?;
        }
    }

    Ok(())
}

fn validate_capsule_request(input: &CreateCapsuleRequest) -> Result<(), ApiError> {
    if input.manifest.title.trim().is_empty()
        || input.manifest.tagline.trim().is_empty()
        || input.manifest.system_prompt.trim().is_empty()
        || input.manifest.result_schema.trim().is_empty()
    {
        return Err(ApiError::bad_request(
            "title, tagline, systemPrompt, and resultSchema are required.",
        ));
    }
    if input.manifest_root.trim().is_empty() {
        return Err(ApiError::bad_request("manifestRoot is required."));
    }
    if input.manifest.creator_wallet.trim().is_empty() {
        return Err(ApiError::bad_request("creatorWallet is required."));
    }
    Ok(())
}

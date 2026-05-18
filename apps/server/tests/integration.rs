use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use skillcapsule_server::{app, config::Config};
use tower::util::ServiceExt;

async fn test_app() -> Router {
    let config = Config {
        bind_addr: "127.0.0.1:0".parse().unwrap(),
        database_url: "sqlite://:memory:".to_string(),
        web_origin: "http://127.0.0.1:5179".to_string(),
        zero_g_router_base_url: "https://router-api-testnet.integratenetwork.work/v1".to_string(),
        zero_g_router_model: "demo-model".to_string(),
        zero_g_router_api_key: None,
        zero_g_chain_rpc_url: None,
        zero_g_chain_id: None,
        skillcapsule_registry_contract: None,
        skillcapsule_private_key: None,
        zero_g_chain_explorer_tx_base: "https://chainscan-galileo.0g.ai/tx/".to_string(),
    };
    app::router(app::build_state(config).await.unwrap())
}

#[tokio::test]
async fn lists_seeded_capsules() {
    let response = test_app()
        .await
        .oneshot(Request::get("/api/capsules").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let text = String::from_utf8(body.to_vec()).unwrap();
    assert!(text.contains("Pitch Doctor"));
    assert!(text.contains("Grant Copilot"));
}

#[tokio::test]
async fn reports_chain_status_without_requiring_chain_config() {
    let response = test_app()
        .await
        .oneshot(
            Request::get("/api/chain/status")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let status: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(
        status.get("readEnabled").unwrap(),
        &serde_json::Value::Bool(false)
    );
    assert_eq!(
        status.get("writeEnabled").unwrap(),
        &serde_json::Value::Bool(false)
    );
}

#[tokio::test]
async fn creates_and_publishes_capsule() {
    let router = test_app().await;
    let payload = serde_json::json!({
        "theme": "editorial ember",
        "manifestRoot": "zg://demo/new-manifest",
        "manifest": {
            "title": "Roadshow Composer",
            "tagline": "Turn scattered notes into a confident roadshow narrative.",
            "systemPrompt": "Write investor-ready messaging.",
            "knowledgeRoots": ["zg://demo/file-1"],
            "resultSchema": "hook: string, bullets: string[]",
            "creatorWallet": "0x1234",
            "agentTokenId": null,
            "coverAssetRoot": null
        }
    });

    let response = router
        .clone()
        .oneshot(
            Request::post("/api/capsules")
                .header("content-type", "application/json")
                .body(Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let created: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let id = created.get("id").unwrap().as_str().unwrap();

    let publish_payload = serde_json::json!({
        "registryContract": "0xregistry",
        "txHash": "0xtx",
        "explorerUrl": "https://chainscan-galileo.0g.ai/tx/0xtx",
        "agentTokenId": "22",
        "version": 2
    });
    let response = router
        .oneshot(
            Request::post(format!("/api/capsules/{id}/publish"))
                .header("content-type", "application/json")
                .body(Body::from(publish_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let published: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(
        published.get("status").unwrap(),
        &serde_json::Value::String("published".to_string())
    );
    assert_eq!(
        published.pointer("/manifest/agentTokenId").unwrap(),
        &serde_json::Value::String("22".to_string())
    );
}

#[tokio::test]
async fn runs_capsule_with_demo_compute() {
    let router = test_app().await;
    let response = router
        .clone()
        .oneshot(Request::get("/api/capsules").body(Body::empty()).unwrap())
        .await
        .unwrap();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let capsules: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let capsule_id = capsules[0].get("id").unwrap().as_str().unwrap();

    let response = router
        .clone()
        .oneshot(
            Request::post(format!("/api/capsules/{capsule_id}/run"))
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "taskInput": "Turn this onchain AI marketplace into a 3 minute hackathon demo."
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let run: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let run_id = run.get("id").unwrap().as_str().unwrap();

    tokio::time::sleep(std::time::Duration::from_millis(60)).await;

    let response = router
        .oneshot(
            Request::get(format!("/api/runs/{run_id}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let run: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(
        run.get("status").unwrap(),
        &serde_json::Value::String("completed".to_string())
    );
    assert!(run.get("outputText").is_some());
}

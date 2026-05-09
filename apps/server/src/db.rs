use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::{Row, SqlitePool, sqlite::SqlitePoolOptions};
use uuid::Uuid;

use crate::models::{
    Capsule, CapsuleManifest, CapsuleRun, CapsuleStatus, CreateCapsuleRequest, ProofRecord,
    RunStatus,
};

pub async fn connect(database_url: &str) -> anyhow::Result<SqlitePool> {
    if let Some(path) = database_url.strip_prefix("sqlite://") {
        if path != ":memory:" && !std::path::Path::new(path).exists() {
            std::fs::File::create(path)?;
        }
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;
    migrate(&pool).await?;
    seed_demo_capsules(&pool).await?;
    Ok(pool)
}

async fn migrate(pool: &SqlitePool) -> sqlx::Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS capsules (
            id TEXT PRIMARY KEY,
            slug TEXT NOT NULL,
            theme TEXT NOT NULL,
            version INTEGER NOT NULL,
            status TEXT NOT NULL,
            manifest_root TEXT NOT NULL,
            manifest_json TEXT NOT NULL,
            registry_contract TEXT,
            publish_tx_hash TEXT,
            explorer_url TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS runs (
            id TEXT PRIMARY KEY,
            capsule_id TEXT NOT NULL,
            status TEXT NOT NULL,
            task_input TEXT NOT NULL,
            output_json TEXT,
            output_text TEXT,
            warnings_json TEXT NOT NULL,
            result_storage_root TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );
        "#,
    )
    .execute(pool)
    .await?;
    Ok(())
}

async fn seed_demo_capsules(pool: &SqlitePool) -> anyhow::Result<()> {
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM capsules")
        .fetch_one(pool)
        .await?;
    if count > 0 {
        return Ok(());
    }

    let seed = vec![
        (
            "Pitch Doctor",
            "Turn a rough project into a sharp pitch, demo script, and 30-second hook.",
            "editorial sunburst",
            "zg://demo/pitch-doctor-manifest",
            "Craft concise founder-grade storytelling from sparse project notes.",
            "hook: string, elevatorPitch: string, demoTalkingPoints: string[], closingLine: string",
        ),
        (
            "Grant Copilot",
            "Package grant and hackathon answers that sound strategic, not generic.",
            "graphite citrus",
            "zg://demo/grant-copilot-manifest",
            "Write application-ready grant responses with technical clarity and outcome framing.",
            "problem: string, applicationSummary: string, milestones: string[], whyNow: string",
        ),
        (
            "Token Explainer",
            "Explain token mechanics in human language and spotlight risk in one pass.",
            "mint evidence",
            "zg://demo/token-explainer-manifest",
            "Translate token mechanics into plain-English narratives with balanced risk analysis.",
            "plainEnglishSummary: string, strengths: string[], risks: string[], recommendedNarrative: string",
        ),
    ];

    for (title, tagline, theme, manifest_root, system_prompt, result_schema) in seed {
        let now = Utc::now();
        let manifest = CapsuleManifest {
            title: title.to_string(),
            tagline: tagline.to_string(),
            system_prompt: system_prompt.to_string(),
            knowledge_roots: vec![
                format!("zg://demo/{}", slugify(title)),
                format!("zg://demo/{}/notes", slugify(title)),
            ],
            result_schema: result_schema.to_string(),
            creator_wallet: "0x8E1eB83B66fD40F642fEA821A3db53C57d497A1d".to_string(),
            agent_token_id: Some("0".to_string()),
            cover_asset_root: Some(format!("zg://demo/{}/cover", slugify(title))),
        };
        let capsule = Capsule {
            id: Uuid::new_v4(),
            slug: slugify(title),
            theme: theme.to_string(),
            version: 1,
            status: CapsuleStatus::Published,
            manifest_root: manifest_root.to_string(),
            manifest,
            registry_contract: Some("0x2700F6A3e505402C9daB154C5c6ab9cAEC98EF1F".to_string()),
            publish_tx_hash: Some(format!("0xdemo{}", &slugify(title))),
            explorer_url: Some(format!(
                "https://chainscan-galileo.0g.ai/tx/0xdemo{}",
                slugify(title)
            )),
            created_at: now,
            updated_at: now,
        };
        write_capsule(pool, &capsule).await?;
    }

    Ok(())
}

pub async fn list_capsules(pool: &SqlitePool) -> anyhow::Result<Vec<Capsule>> {
    let rows = sqlx::query("SELECT * FROM capsules ORDER BY updated_at DESC")
        .fetch_all(pool)
        .await?;
    rows.into_iter().map(read_capsule).collect()
}

pub async fn get_capsule(pool: &SqlitePool, id: Uuid) -> anyhow::Result<Capsule> {
    let row = sqlx::query("SELECT * FROM capsules WHERE id = ?1")
        .bind(id.to_string())
        .fetch_one(pool)
        .await?;
    read_capsule(row)
}

pub async fn create_capsule(
    pool: &SqlitePool,
    input: CreateCapsuleRequest,
) -> anyhow::Result<Capsule> {
    let now = Utc::now();
    let capsule = Capsule {
        id: Uuid::new_v4(),
        slug: slugify(&input.manifest.title),
        theme: input.theme,
        version: 1,
        status: CapsuleStatus::Draft,
        manifest_root: input.manifest_root,
        manifest: input.manifest,
        registry_contract: None,
        publish_tx_hash: None,
        explorer_url: None,
        created_at: now,
        updated_at: now,
    };
    write_capsule(pool, &capsule).await?;
    Ok(capsule)
}

pub async fn publish_capsule(
    pool: &SqlitePool,
    id: Uuid,
    registry_contract: String,
    publish_tx_hash: String,
    explorer_url: String,
    agent_token_id: Option<String>,
    version: Option<i64>,
) -> anyhow::Result<Capsule> {
    let mut capsule = get_capsule(pool, id).await?;
    capsule.status = CapsuleStatus::Published;
    capsule.registry_contract = Some(registry_contract);
    capsule.publish_tx_hash = Some(publish_tx_hash);
    capsule.explorer_url = Some(explorer_url);
    capsule.updated_at = Utc::now();
    if let Some(version) = version {
        capsule.version = version;
    }
    if let Some(agent_token_id) = agent_token_id {
        capsule.manifest.agent_token_id = Some(agent_token_id);
    }
    sqlx::query(
        "UPDATE capsules
         SET version = ?2, status = ?3, manifest_json = ?4, registry_contract = ?5, publish_tx_hash = ?6, explorer_url = ?7, updated_at = ?8
         WHERE id = ?1",
    )
    .bind(id.to_string())
    .bind(capsule.version)
    .bind(capsule.status.to_string())
    .bind(serde_json::to_string(&capsule.manifest)?)
    .bind(&capsule.registry_contract)
    .bind(&capsule.publish_tx_hash)
    .bind(&capsule.explorer_url)
    .bind(capsule.updated_at.to_rfc3339())
    .execute(pool)
    .await?;
    Ok(capsule)
}

pub async fn insert_run(
    pool: &SqlitePool,
    capsule: &Capsule,
    task_input: &str,
) -> anyhow::Result<CapsuleRun> {
    let now = Utc::now();
    let run = CapsuleRun {
        id: Uuid::new_v4(),
        capsule_id: capsule.id,
        capsule_title: capsule.manifest.title.clone(),
        status: RunStatus::Pending,
        task_input: task_input.to_string(),
        output_json: None,
        output_text: None,
        warnings: Vec::new(),
        proof: empty_proof(capsule),
        created_at: now,
        updated_at: now,
    };

    sqlx::query(
        "INSERT INTO runs (id, capsule_id, status, task_input, output_json, output_text, warnings_json, result_storage_root, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, NULL, NULL, ?5, NULL, ?6, ?7)",
    )
    .bind(run.id.to_string())
    .bind(capsule.id.to_string())
    .bind(run.status.to_string())
    .bind(&run.task_input)
    .bind(serde_json::to_string(&run.warnings)?)
    .bind(run.created_at.to_rfc3339())
    .bind(run.updated_at.to_rfc3339())
    .execute(pool)
    .await?;
    Ok(run)
}

pub async fn update_run_status(
    pool: &SqlitePool,
    id: Uuid,
    status: RunStatus,
    output_json: Option<Value>,
    output_text: Option<String>,
    warnings: Vec<String>,
    result_storage_root: Option<String>,
) -> anyhow::Result<()> {
    sqlx::query(
        "UPDATE runs
         SET status = ?2, output_json = ?3, output_text = ?4, warnings_json = ?5, result_storage_root = ?6, updated_at = ?7
         WHERE id = ?1",
    )
    .bind(id.to_string())
    .bind(status.to_string())
    .bind(output_json.map(|value| value.to_string()))
    .bind(output_text)
    .bind(serde_json::to_string(&warnings)?)
    .bind(result_storage_root)
    .bind(Utc::now().to_rfc3339())
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_run(pool: &SqlitePool, id: Uuid) -> anyhow::Result<CapsuleRun> {
    let row = sqlx::query(
        r#"
        SELECT
            runs.*,
            capsules.manifest_json AS capsule_manifest_json,
            capsules.manifest_root AS capsule_manifest_root,
            capsules.registry_contract AS capsule_registry_contract,
            capsules.publish_tx_hash AS capsule_publish_tx_hash,
            capsules.explorer_url AS capsule_explorer_url
        FROM runs
        JOIN capsules ON capsules.id = runs.capsule_id
        WHERE runs.id = ?1
        "#,
    )
    .bind(id.to_string())
    .fetch_one(pool)
    .await?;
    read_run(row)
}

async fn write_capsule(pool: &SqlitePool, capsule: &Capsule) -> anyhow::Result<()> {
    sqlx::query(
        "INSERT INTO capsules (id, slug, theme, version, status, manifest_root, manifest_json, registry_contract, publish_tx_hash, explorer_url, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
    )
    .bind(capsule.id.to_string())
    .bind(&capsule.slug)
    .bind(&capsule.theme)
    .bind(capsule.version)
    .bind(capsule.status.to_string())
    .bind(&capsule.manifest_root)
    .bind(serde_json::to_string(&capsule.manifest)?)
    .bind(&capsule.registry_contract)
    .bind(&capsule.publish_tx_hash)
    .bind(&capsule.explorer_url)
    .bind(capsule.created_at.to_rfc3339())
    .bind(capsule.updated_at.to_rfc3339())
    .execute(pool)
    .await?;
    Ok(())
}

fn read_capsule(row: sqlx::sqlite::SqliteRow) -> anyhow::Result<Capsule> {
    Ok(Capsule {
        id: Uuid::parse_str(row.try_get::<String, _>("id")?.as_str())?,
        slug: row.try_get("slug")?,
        theme: row.try_get("theme")?,
        version: row.try_get("version")?,
        status: CapsuleStatus::try_from(row.try_get::<String, _>("status")?.as_str())?,
        manifest_root: row.try_get("manifest_root")?,
        manifest: serde_json::from_str(row.try_get::<String, _>("manifest_json")?.as_str())?,
        registry_contract: row.try_get("registry_contract")?,
        publish_tx_hash: row.try_get("publish_tx_hash")?,
        explorer_url: row.try_get("explorer_url")?,
        created_at: parse_time(row.try_get::<String, _>("created_at")?.as_str())?,
        updated_at: parse_time(row.try_get::<String, _>("updated_at")?.as_str())?,
    })
}

fn read_run(row: sqlx::sqlite::SqliteRow) -> anyhow::Result<CapsuleRun> {
    let manifest: CapsuleManifest =
        serde_json::from_str(row.try_get::<String, _>("capsule_manifest_json")?.as_str())?;
    let manifest_root: String = row.try_get("capsule_manifest_root")?;
    let output_json = row
        .try_get::<Option<String>, _>("output_json")?
        .map(|value| serde_json::from_str(value.as_str()))
        .transpose()?;
    let output_text: Option<String> = row.try_get("output_text")?;
    let result_storage_root: Option<String> = row.try_get("result_storage_root")?;
    let warnings: Vec<String> =
        serde_json::from_str(row.try_get::<String, _>("warnings_json")?.as_str())?;
    let task_input: String = row.try_get("task_input")?;
    let proof_hash = {
        let mut parts = vec![manifest_root.as_str(), task_input.as_str()];
        if let Some(output_text) = output_text.as_deref() {
            parts.push(output_text);
        }
        crate::ai::build_proof_hash(&parts)
    };

    Ok(CapsuleRun {
        id: Uuid::parse_str(row.try_get::<String, _>("id")?.as_str())?,
        capsule_id: Uuid::parse_str(row.try_get::<String, _>("capsule_id")?.as_str())?,
        capsule_title: manifest.title.clone(),
        status: RunStatus::try_from(row.try_get::<String, _>("status")?.as_str())?,
        task_input,
        output_json,
        output_text,
        warnings,
        proof: ProofRecord {
            manifest_root,
            knowledge_roots: manifest.knowledge_roots,
            cover_asset_root: manifest.cover_asset_root,
            registry_contract: row.try_get("capsule_registry_contract")?,
            publish_tx_hash: row.try_get("capsule_publish_tx_hash")?,
            explorer_url: row.try_get("capsule_explorer_url")?,
            result_storage_root,
            compute_mode: "0G Compute Router".to_string(),
            compute_model: "server-managed".to_string(),
            proof_hash,
        },
        created_at: parse_time(row.try_get::<String, _>("created_at")?.as_str())?,
        updated_at: parse_time(row.try_get::<String, _>("updated_at")?.as_str())?,
    })
}

fn parse_time(value: &str) -> anyhow::Result<DateTime<Utc>> {
    Ok(DateTime::parse_from_rfc3339(value)?.with_timezone(&Utc))
}

fn slugify(value: &str) -> String {
    let mut slug = String::new();
    for character in value.chars() {
        if character.is_ascii_alphanumeric() {
            slug.push(character.to_ascii_lowercase());
        } else if (character.is_whitespace() || character == '-' || character == '_')
            && !slug.ends_with('-')
        {
            slug.push('-');
        }
    }
    slug.trim_matches('-').to_string()
}

fn empty_proof(capsule: &Capsule) -> ProofRecord {
    ProofRecord {
        manifest_root: capsule.manifest_root.clone(),
        knowledge_roots: capsule.manifest.knowledge_roots.clone(),
        cover_asset_root: capsule.manifest.cover_asset_root.clone(),
        registry_contract: capsule.registry_contract.clone(),
        publish_tx_hash: capsule.publish_tx_hash.clone(),
        explorer_url: capsule.explorer_url.clone(),
        result_storage_root: None,
        compute_mode: "0G Compute Router".to_string(),
        compute_model: "pending".to_string(),
        proof_hash: crate::ai::build_proof_hash(&[
            capsule.manifest_root.as_str(),
            capsule.slug.as_str(),
        ]),
    }
}

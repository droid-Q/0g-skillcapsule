use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CapsuleManifest {
    pub title: String,
    pub tagline: String,
    pub system_prompt: String,
    pub knowledge_roots: Vec<String>,
    pub result_schema: String,
    pub creator_wallet: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_token_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_asset_root: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CapsuleStatus {
    Draft,
    Published,
    Paused,
}

impl std::fmt::Display for CapsuleStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Draft => "draft",
            Self::Published => "published",
            Self::Paused => "paused",
        };
        write!(f, "{value}")
    }
}

impl TryFrom<&str> for CapsuleStatus {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "draft" => Ok(Self::Draft),
            "published" => Ok(Self::Published),
            "paused" => Ok(Self::Paused),
            other => anyhow::bail!("unknown capsule status {other}"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RunStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

impl std::fmt::Display for RunStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Pending => "pending",
            Self::Running => "running",
            Self::Completed => "completed",
            Self::Failed => "failed",
        };
        write!(f, "{value}")
    }
}

impl TryFrom<&str> for RunStatus {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "pending" => Ok(Self::Pending),
            "running" => Ok(Self::Running),
            "completed" => Ok(Self::Completed),
            "failed" => Ok(Self::Failed),
            other => anyhow::bail!("unknown run status {other}"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ProofRecord {
    pub manifest_root: String,
    pub knowledge_roots: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_asset_root: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_contract: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_tx_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explorer_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_storage_root: Option<String>,
    pub compute_mode: String,
    pub compute_model: String,
    pub proof_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Capsule {
    pub id: Uuid,
    pub slug: String,
    pub theme: String,
    pub version: i64,
    pub status: CapsuleStatus,
    pub manifest_root: String,
    pub manifest: CapsuleManifest,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_contract: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_tx_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explorer_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CapsuleRun {
    pub id: Uuid,
    pub capsule_id: Uuid,
    pub capsule_title: String,
    pub status: RunStatus,
    pub task_input: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_json: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_text: Option<String>,
    pub warnings: Vec<String>,
    pub proof: ProofRecord,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCapsuleRequest {
    pub theme: String,
    pub manifest_root: String,
    pub manifest: CapsuleManifest,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublishCapsuleRequest {
    pub registry_contract: String,
    pub tx_hash: String,
    pub explorer_url: String,
    #[serde(default)]
    pub agent_token_id: Option<String>,
    #[serde(default)]
    pub version: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRunRequest {
    pub task_input: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChainPublishRequest {
    #[serde(default)]
    pub registry_contract: Option<String>,
    #[serde(default)]
    pub agent_token_id: Option<String>,
    #[serde(default)]
    pub version: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChainStatus {
    pub read_enabled: bool,
    pub write_enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rpc_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_contract: Option<String>,
    pub explorer_tx_base_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChainCapsuleRecord {
    pub capsule_key: String,
    pub owner: String,
    pub manifest_root: String,
    pub agent_token_id: String,
    pub version: u64,
    pub status: CapsuleStatus,
    pub updated_at: u64,
    pub exists: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChainPublishResult {
    pub capsule_key: String,
    pub registry_contract: String,
    pub tx_hash: String,
    pub explorer_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_number: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChainPublishResponse {
    pub capsule: Capsule,
    pub chain: ChainPublishResult,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ApiErrorBody {
    pub error: String,
}

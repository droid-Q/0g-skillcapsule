use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::models::{CapsuleManifest, ProofRecord};

#[derive(Clone)]
pub struct ZeroGComputeClient {
    client: reqwest::Client,
    base_url: String,
    model: String,
    api_key: Option<String>,
}

pub struct AiRunOutput {
    pub output_json: Value,
    pub output_text: String,
    pub warnings: Vec<String>,
}

impl ZeroGComputeClient {
    pub fn new(base_url: String, model: String, api_key: Option<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url,
            model,
            api_key,
        }
    }

    pub fn enabled(&self) -> bool {
        self.api_key
            .as_deref()
            .is_some_and(|value| !value.is_empty())
    }

    pub fn model(&self) -> &str {
        &self.model
    }

    pub async fn run_capsule(
        &self,
        manifest: &CapsuleManifest,
        task_input: &str,
    ) -> anyhow::Result<AiRunOutput> {
        if !self.enabled() {
            return Ok(self.demo_output(manifest, task_input));
        }

        let url = format!("{}/chat/completions", self.base_url.trim_end_matches('/'));
        let system = format!(
            "{}\n\nReturn JSON only. Follow this result schema guidance exactly:\n{}",
            manifest.system_prompt, manifest.result_schema
        );
        let user = format!(
            "Capsule title: {}\nTagline: {}\nCreator wallet: {}\nKnowledge roots: {}\nTask input: {}\nReturn compact JSON that is ready to show in a demo UI.",
            manifest.title,
            manifest.tagline,
            manifest.creator_wallet,
            manifest.knowledge_roots.join(", "),
            task_input
        );

        let mut request = self.client.post(url).json(&serde_json::json!({
            "model": self.model,
            "response_format": { "type": "json_object" },
            "messages": [
                { "role": "system", "content": system },
                { "role": "user", "content": user }
            ]
        }));
        if let Some(api_key) = &self.api_key {
            request = request.bearer_auth(api_key);
        }

        let value: Value = request.send().await?.error_for_status()?.json().await?;
        let text = value
            .pointer("/choices/0/message/content")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow::anyhow!("0G router response did not include content"))?;
        let output_json = serde_json::from_str::<Value>(text).unwrap_or_else(|_| {
            serde_json::json!({
                "summary": text,
                "nextAction": "Review the raw model output because it was not valid JSON."
            })
        });
        Ok(AiRunOutput {
            output_text: serde_json::to_string_pretty(&output_json)?,
            output_json,
            warnings: Vec::new(),
        })
    }

    fn demo_output(&self, manifest: &CapsuleManifest, task_input: &str) -> AiRunOutput {
        let result = if manifest.title.contains("Pitch") {
            serde_json::json!({
                "hook": "We turn onchain proof into a trust layer for AI work products.",
                "elevatorPitch": format!("{} helps teams convert rough ideas like '{}' into investor-ready language with verifiable provenance on 0G.", manifest.title, task_input),
                "demoTalkingPoints": [
                    "Knowledge pack lives on 0G Storage",
                    "Inference runs through the 0G Compute Router",
                    "Publishing creates explorer-visible proof"
                ],
                "closingLine": "It feels like an app you can ship, not a weekend prototype."
            })
        } else if manifest.title.contains("Grant") {
            serde_json::json!({
                "problem": "Reviewers struggle to verify whether submissions are original, complete, and technically grounded.",
                "applicationSummary": format!("Generated from the task '{}'.", task_input),
                "milestones": [
                    "Ship an explorable marketplace of reusable agent capsules",
                    "Show manifest and result proof on 0G",
                    "Package a 3-minute demo with reproducible local steps"
                ],
                "whyNow": "Hackathon judges reward technical integration plus clear product value."
            })
        } else {
            serde_json::json!({
                "plainEnglishSummary": format!("{} turns the task '{}' into a token design explanation with risk framing.", manifest.title, task_input),
                "strengths": [
                    "Clear utility narrative",
                    "Reusable creator workflow",
                    "Proof-first UX"
                ],
                "risks": [
                    "Token incentive loop may be under-specified",
                    "Governance details need stronger anti-sybil framing"
                ],
                "recommendedNarrative": "Lead with creator trust, verifiable AI execution, and portable capsule identity."
            })
        };

        AiRunOutput {
            output_text: serde_json::to_string_pretty(&result)
                .unwrap_or_else(|_| "{\"summary\":\"demo\"}".to_string()),
            output_json: result,
            warnings: vec![
                "Using local demo output because ZERO_G_ROUTER_API_KEY is not configured."
                    .to_string(),
            ],
        }
    }
}

pub fn build_proof_hash(parts: &[&str]) -> String {
    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update(part.as_bytes());
    }
    format!("sha256:{}", hex::encode(hasher.finalize()))
}

pub fn build_proof_record(
    manifest: &CapsuleManifest,
    manifest_root: &str,
    registry_contract: Option<String>,
    publish_tx_hash: Option<String>,
    explorer_url: Option<String>,
    result_storage_root: Option<String>,
    model: &str,
    task_input: &str,
    output_text: &str,
) -> ProofRecord {
    let proof_hash = build_proof_hash(&[
        manifest_root,
        &manifest.knowledge_roots.join("|"),
        task_input,
        output_text,
    ]);

    ProofRecord {
        manifest_root: manifest_root.to_string(),
        knowledge_roots: manifest.knowledge_roots.clone(),
        cover_asset_root: manifest.cover_asset_root.clone(),
        registry_contract,
        publish_tx_hash,
        explorer_url,
        result_storage_root,
        compute_mode: "0G Compute Router".to_string(),
        compute_model: model.to_string(),
        proof_hash,
    }
}

use std::{str::FromStr, sync::Arc, time::Duration};

use ethers::{prelude::*, utils::keccak256};

use crate::models::{Capsule, CapsuleStatus, ChainCapsuleRecord, ChainPublishResult, ChainStatus};

abigen!(
    SkillCapsuleRegistryContract,
    r#"[
        function upsertCapsule(bytes32 capsuleKey,string manifestRoot,uint256 agentTokenId,uint64 version,uint8 status)
        function setCapsuleStatus(bytes32 capsuleKey,uint8 status)
        function getCapsule(bytes32 capsuleKey) view returns (tuple(address owner,string manifestRoot,uint256 agentTokenId,uint64 version,uint8 status,uint64 updatedAt))
    ]"#
);

#[derive(Clone)]
pub struct ChainClient {
    rpc_url: Option<String>,
    chain_id: Option<u64>,
    registry_contract: Option<String>,
    private_key: Option<String>,
    explorer_tx_base_url: String,
}

impl ChainClient {
    pub fn new(
        rpc_url: Option<String>,
        chain_id: Option<u64>,
        registry_contract: Option<String>,
        private_key: Option<String>,
        explorer_tx_base_url: String,
    ) -> Self {
        Self {
            rpc_url: clean_optional(rpc_url),
            chain_id,
            registry_contract: clean_optional(registry_contract),
            private_key: clean_optional(private_key),
            explorer_tx_base_url,
        }
    }

    pub fn status(&self) -> ChainStatus {
        ChainStatus {
            read_enabled: self.read_enabled(),
            write_enabled: self.write_enabled(),
            rpc_url: self.rpc_url.clone(),
            chain_id: self.chain_id,
            registry_contract: self.registry_contract.clone(),
            explorer_tx_base_url: self.explorer_tx_base_url.clone(),
        }
    }

    pub fn read_enabled(&self) -> bool {
        self.rpc_url.is_some() && self.registry_contract.is_some()
    }

    pub fn write_enabled(&self) -> bool {
        self.read_enabled() && self.private_key.is_some()
    }

    pub fn capsule_key(capsule: &Capsule) -> String {
        format!("0x{}", hex::encode(capsule_key_bytes(capsule)))
    }

    pub async fn get_capsule(&self, capsule: &Capsule) -> anyhow::Result<ChainCapsuleRecord> {
        let contract = self
            .read_contract(self.registry_contract.as_deref())
            .await?;
        let record = contract
            .get_capsule(capsule_key_bytes(capsule))
            .call()
            .await?;
        Ok(ChainCapsuleRecord {
            capsule_key: Self::capsule_key(capsule),
            owner: format!("{:#x}", record.0),
            manifest_root: record.1,
            agent_token_id: record.2.to_string(),
            version: record.3,
            status: status_from_u8(record.4),
            updated_at: record.5,
            exists: record.0 != Address::zero(),
        })
    }

    pub async fn publish_capsule(
        &self,
        capsule: &Capsule,
        registry_contract: Option<&str>,
        version: u64,
        agent_token_id: U256,
    ) -> anyhow::Result<ChainPublishResult> {
        let registry_contract = registry_contract
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned)
            .or_else(|| self.registry_contract.clone())
            .ok_or_else(|| {
                anyhow::anyhow!("SKILLCAPSULE_REGISTRY_CONTRACT_ADDRESS is not configured")
            })?;
        let contract = self.write_contract(&registry_contract).await?;
        let call = contract.upsert_capsule(
            capsule_key_bytes(capsule),
            capsule.manifest_root.clone(),
            agent_token_id,
            version,
            status_to_u8(&CapsuleStatus::Published),
        );
        let pending_tx = call.send().await?;
        let tx_hash = format!("{:#x}", pending_tx.tx_hash());
        let receipt = pending_tx.await?;
        let block_number =
            receipt.and_then(|receipt| receipt.block_number.map(|value| value.as_u64()));

        Ok(ChainPublishResult {
            capsule_key: Self::capsule_key(capsule),
            registry_contract,
            tx_hash: tx_hash.clone(),
            explorer_url: self.explorer_url(&tx_hash),
            block_number,
        })
    }

    pub fn parse_agent_token_id(value: Option<&str>) -> anyhow::Result<U256> {
        let value = value
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .unwrap_or("0");
        U256::from_dec_str(value)
            .or_else(|_| U256::from_str(value))
            .map_err(|error| anyhow::anyhow!("invalid agentTokenId: {error}"))
    }

    async fn provider(&self) -> anyhow::Result<Provider<Http>> {
        let rpc_url = self
            .rpc_url
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("ZERO_G_CHAIN_RPC_URL is not configured"))?;
        Ok(Provider::<Http>::try_from(rpc_url)?.interval(Duration::from_millis(250)))
    }

    async fn read_contract(
        &self,
        registry_contract: Option<&str>,
    ) -> anyhow::Result<SkillCapsuleRegistryContract<Provider<Http>>> {
        let provider = self.provider().await?;
        let address = parse_address(registry_contract.or(self.registry_contract.as_deref()))?;
        Ok(SkillCapsuleRegistryContract::new(
            address,
            Arc::new(provider),
        ))
    }

    async fn write_contract(
        &self,
        registry_contract: &str,
    ) -> anyhow::Result<SkillCapsuleRegistryContract<SignerMiddleware<Provider<Http>, LocalWallet>>>
    {
        let provider = self.provider().await?;
        let chain_id = match self.chain_id {
            Some(chain_id) => chain_id,
            None => provider.get_chainid().await?.as_u64(),
        };
        let private_key = self
            .private_key
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("SKILLCAPSULE_PRIVATE_KEY is not configured"))?;
        let wallet = private_key.parse::<LocalWallet>()?.with_chain_id(chain_id);
        let client = SignerMiddleware::new(provider, wallet);
        let address = parse_address(Some(registry_contract))?;
        Ok(SkillCapsuleRegistryContract::new(address, Arc::new(client)))
    }

    fn explorer_url(&self, tx_hash: &str) -> String {
        format!(
            "{}/{}",
            self.explorer_tx_base_url.trim_end_matches('/'),
            tx_hash
        )
    }
}

fn capsule_key_bytes(capsule: &Capsule) -> [u8; 32] {
    keccak256(capsule.id.to_string().as_bytes())
}

fn clean_optional(value: Option<String>) -> Option<String> {
    value
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn parse_address(value: Option<&str>) -> anyhow::Result<Address> {
    let value = value.ok_or_else(|| anyhow::anyhow!("registry contract is not configured"))?;
    Address::from_str(value)
        .map_err(|error| anyhow::anyhow!("invalid registry contract address: {error}"))
}

fn status_to_u8(status: &CapsuleStatus) -> u8 {
    match status {
        CapsuleStatus::Draft => 0,
        CapsuleStatus::Published => 1,
        CapsuleStatus::Paused => 2,
    }
}

fn status_from_u8(status: u8) -> CapsuleStatus {
    match status {
        1 => CapsuleStatus::Published,
        2 => CapsuleStatus::Paused,
        _ => CapsuleStatus::Draft,
    }
}

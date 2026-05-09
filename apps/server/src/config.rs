use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct Config {
    pub bind_addr: SocketAddr,
    pub database_url: String,
    pub web_origin: String,
    pub zero_g_router_base_url: String,
    pub zero_g_router_model: String,
    pub zero_g_router_api_key: Option<String>,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            bind_addr: std::env::var("BIND_ADDR")
                .unwrap_or_else(|_| "127.0.0.1:18089".to_string())
                .parse()?,
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite://0g-skillcapsule.db".to_string()),
            web_origin: std::env::var("WEB_ORIGIN")
                .unwrap_or_else(|_| "http://127.0.0.1:5179".to_string()),
            zero_g_router_base_url: std::env::var("ZERO_G_ROUTER_BASE_URL").unwrap_or_else(|_| {
                "https://router-api-testnet.integratenetwork.work/v1".to_string()
            }),
            zero_g_router_model: std::env::var("ZERO_G_ROUTER_MODEL")
                .unwrap_or_else(|_| "moonshotai/Kimi-K2-Instruct".to_string()),
            zero_g_router_api_key: std::env::var("ZERO_G_ROUTER_API_KEY").ok(),
        })
    }
}

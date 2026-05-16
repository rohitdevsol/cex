use figment::{
    Figment,
    providers::{Env, Serialized},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    #[serde(rename = "API_PORT")]
    pub api_port: u16,
    #[serde(rename = "DATABASE_URL")]
    pub database_url: String,
    #[serde(rename = "RUST_LOG")]
    pub rust_log: String,
}

impl Config {
    pub fn load() -> Self {
        dotenvy::dotenv().ok();

        Figment::new()
            .merge(Serialized::defaults(Config {
                api_port: 8080,
                database_url: "postgres://postgres:postgres@localhost:5432/cex".to_string(),
                rust_log: "debug".to_string(),
            }))
            .merge(Env::raw())
            .extract()
            .expect("failed to load config")
    }
}

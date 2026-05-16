#![allow(dead_code, unused)]
use actix_web::{App, HttpServer, web};
use config::Config;
use db::create_pool;

use crate::state::AppState;
mod routes;
mod state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::load();

    tracing_subscriber::fmt().with_env_filter(&config.rust_log).init();
    tracing::info!("starting api server");

    let pool = create_pool(&config.database_url).await.expect("failed to connect to database");
    tracing::info!("database connection established");

    let state = web::Data::new(AppState { db: pool, config: config.clone() });
    HttpServer::new(move || App::new().app_data(state.clone()).service(routes::health))
        .bind(("127.0.0.1", config.api_port))?
        .run()
        .await
}

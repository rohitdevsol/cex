use actix_web::{App, HttpServer};

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt().with_env_filter("debug").init();

    tracing::info!("starting api server");

    HttpServer::new(|| App::new().service(routes::health)).bind(("127.0.0.1", 8080))?.run().await
}

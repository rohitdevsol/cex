use actix_web::{HttpResponse, Responder, get, web};
use serde_json::json;
use sqlx::query_scalar;

use crate::state::AppState;

#[get("/health")]
pub async fn health(state: web::Data<AppState>) -> impl Responder {
    let db_status = query_scalar::<_, i32>("SELECT 1").fetch_one(&state.db).await;

    match db_status {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": "ok",
            "service": "api",
        })),
        Err(err) => {
            tracing::error!("database healthcheck failed: {:?}", err);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "database": "disconnected",
            }))
        }
    }
}

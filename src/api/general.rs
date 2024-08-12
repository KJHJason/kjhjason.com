use crate::models::index::Index;

use actix_web::{get, HttpResponse};

#[get("/api")]
async fn api_index() -> HttpResponse {
    let serialised = serde_json::to_string_pretty(&Index::new()).unwrap();
    return HttpResponse::Ok()
        .content_type("application/json")
        .body(serialised);
}

#[get("/api/health")]
async fn api_health() -> HttpResponse {
    return HttpResponse::Ok()
        .content_type("application/json")
        .body(r#"{"status": "ok"}"#);
}

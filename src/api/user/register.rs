use actix_web::{HttpRequest, HttpResponse, Responder, web};
use redis::Client;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
#[derive(Deserialize, Serialize)]
struct NewUser {
    username: String,
    mail: String,
    password: String,
}
#[actix_web::post("/api/user/register")]
async fn register(
    new_user: web::Json<NewUser>,
    req: HttpRequest,
    redis_client: web::Data<Client>,
    postgres_client: web::Data<PgPool>,
) -> impl Responder {
    let user_agent = req
        .headers()
        .get("User-Agent")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("unknown");

    HttpResponse::Ok().body(format!("Tu User-Agent es: {}", user_agent))
}

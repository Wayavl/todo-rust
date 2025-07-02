use actix_web::{HttpResponse, Responder, web};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use once_cell::sync::Lazy;
use redis::{AsyncCommands, aio::MultiplexedConnection};
use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

// Regex
static USERNAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("^[a-zA-Z0-9]{3,20}$").unwrap());
static MAIL_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[a-zA-Z\-\.]+@([a-zA-Z\-]+\.)+[a-zA-Z\-]{2,4}$").unwrap());
static PASSWORD_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("^[a-zA-Z0-9]{3,64}$").unwrap());
// Query
const REGISTER_USER: &str =
    "INSERT INTO users (username, password, mail) VALUES ($1, $2, $3) RETURNING id";

#[derive(Deserialize, Serialize)]
struct NewUser {
    username: String,
    mail: String,
    password: String,
}
impl NewUser {
    pub fn valid(&self) -> bool {
        if !USERNAME_REGEX.is_match(&self.username) {
            return false;
        }
        if !MAIL_REGEX.is_match(&self.mail) {
            return false;
        }
        if !PASSWORD_REGEX.is_match(&self.password) {
            return false;
        }
        true
    }
    pub fn encrypt_password(&self) -> Result<String, argon2::password_hash::Error> {
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let hash = argon2.hash_password(self.password.as_bytes(), &salt)?;
        Ok(hash.to_string())
    }
}
#[actix_web::post("/api/user/register")]
pub async fn register(
    new_user: web::Json<NewUser>,
    redis_connection: web::Data<MultiplexedConnection>,
    postgres_client: web::Data<PgPool>,
) -> impl Responder {
    if !new_user.valid() {
        return HttpResponse::UnprocessableEntity().body(format!("Invalid values"));
    }
    // Save to user,mail,passwd
    let mut transaction = match postgres_client.begin().await {
        Ok(v) => v,
        Err(_e) => return HttpResponse::InternalServerError().body("Database had an error."),
    };
    let hashed_password = match new_user.encrypt_password() {
        Ok(v) => v,
        Err(_e) => {
            return HttpResponse::InternalServerError().body("Error during password encryption.");
        }
    };
    let id: i32 = match sqlx::query_scalar(REGISTER_USER)
        .bind(&new_user.username)
        .bind(hashed_password)
        .bind(&new_user.mail)
        .fetch_one(&mut *transaction)
        .await
    {
        Ok(v) => v,
        Err(_e) => {
            return HttpResponse::InternalServerError().body("Error during database execution");
        }
    };
    match transaction.commit().await {
        Ok(i) => i,
        Err(_e) => return HttpResponse::InternalServerError().body("Database error during commit"),
    };
    // Now the implementation of Redis.
    // Format: `session:<uuid> = user_id`
    let uuid = Uuid::new_v4().to_string();
    let _: () = match redis_connection
        .set(format!("session:{}", uuid.to_string()), id)
        .await
    {
        Ok(v) => v,
        Err(_e) => return HttpResponse::InternalServerError().body("Redis error to insert"),
    };
    // Placeholder for successful registration logic
    HttpResponse::Ok().body("User registered successfully")
}

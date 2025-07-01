mod api;
use actix_web::{App, HttpServer, web};
use std::error::Error;

use crate::api::user::register::register;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let port = 8080;
    let redis_client = redis::Client::open("redis://localhost:6379")
        .expect("Couldn't open redis database connection.");
    let posgres_client = sqlx::PgPool::connect("postgres://wajavi:1234@127.0.0.1:5432/mydb")
        .await
        .expect("Couldn't open Postgres SQL connection.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(redis_client.clone()))
            .app_data(web::Data::new(posgres_client.clone()))
            .service(register)
    })
    .bind(("127.0.0.1", port))?
    //.workers() // Cantidad de hilos DEFAULT Nucleos
    .run()
    .await?;

    Ok(())
}

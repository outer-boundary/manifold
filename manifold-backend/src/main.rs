use actix_web::{App, HttpServer};
use sqlx::mysql;
use std::env;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::from_filename(".env")?;

    let database_url = env::var("DATABASE_URL")?;
    let pool = mysql::MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    HttpServer::new(|| App::new())
        .bind(("127.0.0.1", 8080))?
        .run()
        .await?;

    Ok(())
}

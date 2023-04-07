use actix_web::{App, HttpServer};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_filename(".env").ok();

    let url = env::var("DATABASE_URL").expect("DATABASE_URL env var could not be found");
    println!("{url}");

    HttpServer::new(|| App::new())
        .bind(("127.0.0.1", 8080))?
        .run()
        .await?;

    Ok(())
}

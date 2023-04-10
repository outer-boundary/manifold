use actix_web::{web::Data, App, HttpServer};

mod routes;
use routes::messages::{add_message, get_messages};

mod models;
mod util;
use util::environment;

type Error = Box<dyn std::error::Error>;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    let config = environment::init().await?;

    println!(
        "{} server started at {}:{}",
        config.env, config.server.url.host, config.server.url.port
    );

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(config.db.pool.clone()))
            .service(get_messages)
            .service(add_message)
    })
    .bind((config.server.url.host, config.server.url.port))?
    .run()
    .await?;

    Ok(())
}

use actix_web::{web::Data, App, HttpServer};

mod models;
mod routes;
mod util;

use routes::health_check::health_check;
use routes::messages::messages_scope;
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
            .service(health_check)
            .configure(messages_scope)
    })
    .bind((config.server.url.host, config.server.url.port))?
    .run()
    .await?;

    Ok(())
}

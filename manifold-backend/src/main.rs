mod models;
mod routes;
mod util;

use crate::util::database::connect_db;
use actix_web::{web::Data, App, HttpServer};
use routes::health_check::health_check;
use routes::messages::messages_scope;
use sqlx::MySqlPool;
use util::environment;

type Error = Box<dyn std::error::Error>;

#[derive(Clone)]
struct AppState {
    pool: MySqlPool,
}

#[actix_web::main]
async fn main() -> Result<(), Error> {
    let config = environment::init().await?;

    let app_state = AppState {
        pool: connect_db(&config.db.url, None).await?,
    };

    // Apply the migrations to the database before running the server.
    sqlx::migrate!().run(&app_state.pool).await?;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(app_state.clone()))
            .service(health_check)
            .configure(messages_scope)
    })
    .bind((config.server.url.host, config.server.url.port))?
    .run()
    .await?;

    Ok(())
}

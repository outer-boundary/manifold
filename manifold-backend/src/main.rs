extern crate argonautica;

mod models;
mod routes;
mod util;

use crate::util::database::connect_db;
use actix_web::web::scope;
use actix_web::{web::Data, App, HttpServer};
use routes::health_check::health_check;
use routes::users::users_scope;
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

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(app_state.clone()))
            .service(scope("/api").service(health_check).configure(users_scope))
    })
    .bind(config.server.url)?
    .run()
    .await?;

    Ok(())
}

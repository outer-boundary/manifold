extern crate argonautica;

use actix_web::web::scope;
use actix_web::{web::Data, App, HttpServer};
use backend::common::{AppState, Error};
use backend::routes::{health_check::health_check, users::users_scope};
use backend::util::database::connect_db;
use backend::util::environment;

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

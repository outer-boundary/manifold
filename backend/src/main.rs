extern crate argonautica;

use actix_web::web::scope;
use actix_web::{web::Data, App, HttpServer};
use backend::common::{AppState, Error};
use backend::routes::{health_check::health_check_route, users::users_scope};
use backend::util::database::connect_db;
use backend::util::environment;
use backend::util::telemetry;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    let config = environment::init().await?;

    let subscriber = telemetry::get_subscriber(config.env);
    telemetry::init_subscriber(subscriber);

    let app_state = AppState {
        pool: connect_db(&config.db.url, None).await?,
    };

    tracing::info!(target: "backend", "Listening on http://{}:{}", config.server.url.0, config.server.url.1);

    HttpServer::new(move || {
        App::new().app_data(Data::new(app_state.clone())).service(
            scope("/api")
                .service(health_check_route)
                .configure(users_scope),
        )
    })
    .bind(config.server.url)?
    .run()
    .await?;

    Ok(())
}

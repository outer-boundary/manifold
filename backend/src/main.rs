extern crate argonautica;

use backend::{
    common::Error,
    startup::Application,
    util::{configuration::get_config, telemetry},
};

#[actix_web::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv().ok();

    let config = get_config().expect("Failed to read settings");

    let subscriber = telemetry::get_subscriber(config.environment.clone());
    telemetry::init_subscriber(subscriber);

    let application = Application::build(config.clone()).await?;

    tracing::info!(target: "backend", "Listening on {}://{}:{}", config.server.scheme, config.server.port, config.server.host);

    application.run_until_stopped().await?;

    Ok(())
}

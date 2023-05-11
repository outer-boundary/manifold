use crate::{
    common::{AppState, Error},
    routes::{health_check::health_check_route, users::users_scope},
    util::{configuration::Configuration, database::connect_db},
};
use actix_web::{
    dev::Server,
    web::{scope, Data},
    App, HttpServer,
};
use std::net::TcpListener;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(config: Configuration) -> Result<Self, Error> {
        let address = format!("{}:{}", config.server.host, config.server.port);

        let listener = TcpListener::bind(&address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(listener, config).await?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), Error> {
        self.server.await.map_err(|err| err.into())
    }
}

async fn run(listener: TcpListener, config: Configuration) -> Result<Server, Error> {
    let app_state = AppState {
        pool: connect_db(&config.database.url, None).await?,
    };

    let server = HttpServer::new(move || {
        App::new().app_data(Data::new(app_state.clone())).service(
            scope("/api")
                .service(health_check_route)
                .configure(users_scope),
        )
    })
    .listen(listener)?
    .run();

    Ok(server)
}

use super::configuration::{Configuration, DatabaseConfiguration, ServerConfiguration};
use crate::Error;
use once_cell::sync::Lazy;
use std::{env, fmt::Display};

#[derive(Eq, PartialEq)]
pub enum Environment {
    Production,
    Development,
}

impl TryFrom<&str> for Environment {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, self::Error> {
        match value.to_ascii_lowercase().as_str() {
            "production" | "prod" => Ok(Environment::Production),
            "development" | "dev" => Ok(Environment::Development),
            _ => Err(format!("Invalid environment value: {}", value).into()),
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, self::Error> {
        Environment::try_from(value.as_str())
    }
}

impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Environment::Production => "production",
            Environment::Development => "development",
        })
    }
}

pub async fn init() -> Result<Configuration, Error> {
    dotenv::dotenv()?;

    let environment =
        Environment::try_from(env::var("ENVIRONMENT").unwrap_or_else(|_| "development".into()))?;

    match environment {
        Environment::Development => {
            env::set_var("RUST_LOG", "debug");
            static INIT_LOGGER: Lazy<()> = Lazy::new(env_logger::init);
            let _ = &*INIT_LOGGER;
        }
        Environment::Production => (),
    }

    Ok(Configuration {
        env: environment,
        db: DatabaseConfiguration {
            url: env::var("DATABASE_URL")?,
        },
        server: ServerConfiguration {
            url: (
                env::var("SERVER_ADDRESS")?,
                env::var("SERVER_PORT")?.parse()?,
            ),
        },
    })
}

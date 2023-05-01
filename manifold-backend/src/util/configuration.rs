use super::environment::Environment;

pub struct Configuration {
    pub env: Environment,
    pub db: DatabaseConfiguration,
    pub server: ServerConfiguration,
}

pub struct DatabaseConfiguration {
    pub url: String,
}

pub struct ServerConfiguration {
    pub url: (String, u16),
}

use super::{database::DBPool, url::Url};

pub struct Configuration {
    pub env: String,
    pub db: DatabaseConfiguration,
    pub server: ServerConfiguration,
}

pub struct DatabaseConfiguration {
    pub pool: DBPool,
}

pub struct ServerConfiguration {
    pub url: Url,
}

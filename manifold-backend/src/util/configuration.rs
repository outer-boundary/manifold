use super::url::Url;

pub struct Configuration {
    pub env: String,
    pub db: DatabaseConfiguration,
    pub server: ServerConfiguration,
}

pub struct DatabaseConfiguration {
    pub url: String,
}

pub struct ServerConfiguration {
    pub url: Url,
}

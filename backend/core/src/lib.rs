use minijinja::path_loader;

pub mod models;
pub mod routes;
pub mod startup;
pub mod types;
pub mod util;

pub static ENV: once_cell::sync::Lazy<minijinja::Environment<'static>> =
    once_cell::sync::Lazy::new(|| {
        let mut env = minijinja::Environment::new();
        env.set_loader(path_loader("templates"));
        env
    });

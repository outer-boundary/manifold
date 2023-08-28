use minijinja::path_loader;

#[macro_use]
pub mod models;
#[macro_use]
pub mod routes;
#[macro_use]
pub mod startup;
#[macro_use]
pub mod types;
#[macro_use]
pub mod util;

pub static ENV: once_cell::sync::Lazy<minijinja::Environment<'static>> =
    once_cell::sync::Lazy::new(|| {
        let mut env = minijinja::Environment::new();
        env.set_loader(path_loader("templates"));
        env
    });

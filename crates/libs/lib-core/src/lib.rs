mod config;
pub mod ctx;
pub mod model;

// #[cfg(test)]
pub mod _dev_utls;

pub use config::core_config;

#[derive(Clone)]
pub(crate) struct Db;

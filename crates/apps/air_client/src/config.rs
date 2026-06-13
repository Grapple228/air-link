//! Crate config

use crate::error::{Error, Result};
use std::sync::OnceLock;

static INSTANCE: OnceLock<Config> = OnceLock::new();

pub fn config() -> &'static Config {
    INSTANCE.get_or_init(|| {
        Config::load_from_env()
            .unwrap_or_else(|ex| panic!("FATAL - WHOLE LOADING CONF - Cause: {ex:?}"))
    })
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Config {
    pub ADDRESS: std::net::SocketAddr,
    pub WIDTH: u32,
    pub HEIGHT: u32,
}

impl Config {
    fn load_from_env() -> Result<Self> {
        let resolution = grapple_utils::envs::get("RESOLUTION").unwrap_or("1920x1080".to_string());
        let Some((width_str, height_str)) = resolution.split_once('x') else {
            panic!("'RESOLUTION' should be provided in format 'WIDTHxHEIGHT'")
        };

        Ok(Self {
            ADDRESS: grapple_utils::envs::get_parse("ADDRESS").unwrap(),
            WIDTH: width_str.parse().expect("'WIDTH' should be a u32 number"),
            HEIGHT: height_str.parse().expect("'HEIGHT' should be a u32 number"),
        })
    }

    pub fn init_from(cfg: Self) -> Result<()> {
        INSTANCE
            .set(cfg)
            .map_err(|_| Error::ConfigAlreadyInitialized)
    }
}

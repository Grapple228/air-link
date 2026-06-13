//! Crate config

use crate::error::{Error, Result};
use std::{net::SocketAddr, sync::OnceLock};

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
    pub ADDRESS: SocketAddr,
}

impl Config {
    fn load_from_env() -> Result<Self> {
        Ok(Self {
            ADDRESS: grapple_utils::envs::get_parse("ADDRESS")
                .unwrap_or("192.168.0.151:54321".parse().unwrap()),
        })
    }

    pub fn init_from(cfg: Self) -> Result<()> {
        INSTANCE
            .set(cfg)
            .map_err(|_| Error::ConfigAlreadyInitialized)
    }
}

// region:    --- Modules
mod dev_db;

use crate::ctx::Ctx;
use crate::model::{self, ModelManager};
use grapple_utils::cuuid::uuid::Uuid;
use tokio::sync::OnceCell;
use tracing::info;

// endregion: --- Modules

/// Initialize environment for local development
/// (for early development, will be called from main())
pub async fn init_dev() {
    static INIT: OnceCell<()> = OnceCell::const_new();

    INIT.get_or_init(|| async {
        info!("{:<12} - init_dev_all()", "FOR-DEV-ONLY");

        dev_db::init_dev_db().await.unwrap();
    })
    .await;
}

/// Initialize test environment
pub async fn init_test() -> ModelManager {
    static INIT: OnceCell<ModelManager> = OnceCell::const_new();

    let mm = INIT
        .get_or_init(|| async {
            init_dev().await;
            ModelManager::new().await.unwrap()
        })
        .await;

    mm.clone()
}

// region:    --- Seed functions

pub async fn seed_test(_ctx: &Ctx, _mm: &ModelManager, names: &[&str]) -> model::Result<Vec<Uuid>> {
    // Seed something
    let elems = Vec::new();

    for _name in names {}

    Ok(elems)
}

// endregion: --- Seed functions

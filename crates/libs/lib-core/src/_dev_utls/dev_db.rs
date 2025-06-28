use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::Db;
use tracing::info;

// type Db = Pool<Postgres>;

// NOTE: Hardcode to prevent deployed system db update
const PG_DEV_POSTGRES_URL: &str = "postgres://postgres:welcome@localhost:5434/postgres";
const PG_DEV_APP_URL: &str = "postgres://app_user:dev_only_pwd@localhost:5434/app_db";

// pub type Db = Pool<Postgres>;

pub async fn init_dev_db() -> Result<(), Box<dyn std::error::Error>> {
    info!("{:<12} - init_dev_db()", "FOR-DEV-ONLY");

    {
        // Operations over root db
        let _db = new_db_pool(PG_DEV_POSTGRES_URL).await?;
        // Create user `app_user`
    }

    // Operations over app db
    let _app_db = new_db_pool(PG_DEV_APP_URL).await?;
    // Create tables and etc

    // -- Init model layer
    let _mm = ModelManager::new().await?;
    let _ctx = Ctx::root_ctx();

    // Set data using root_ctx

    // Setup initial data
    //let _ctx = Ctx::user_id(Uuid::new_v4());
    // Create data for specific user

    Ok(())
}

async fn new_db_pool(_db_con_url: &str) -> Result<Db, std::io::Error> {
    // PgPoolOptions::new()
    // 	.max_connections(1)
    // 	.acquire_timeout(Duration::from_millis(500))
    // 	.connect(db_con_url)
    // 	.await
    Ok(Db)
}

use service::init;
use service::{Error, Result};

#[tokio::main]
async fn main() -> Result<()> {
    service::init();

    println!("quick-dev");

    Ok(())
}

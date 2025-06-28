use service::Result;

#[tokio::main]
async fn main() -> Result<()> {
    service::init()?;

    // App logic

    Ok(())
}

mod config;
use anyhow::Result;
use email_newsletter::run;

#[tokio::main]
async fn main() -> Result<()> {
    let config = config::Settings::from_yaml().await?;
    let address = format!("127.0.0.1:{}", config.app_port);

    run(&address)?.await?;
    Ok(())
}

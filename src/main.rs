mod config;
use anyhow::Result;
use email_newsletter::run;
use env_logger::Env;
use log::info;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config = config::Settings::from_yaml().await?;
    let address = format!("127.0.0.1:{}", config.app_port);

    let connection;
    loop {
        info!("Attempting to connect to the database...");
        let c = config.database_settings.connect().await;
        match c {
            Ok(conn) => {
                connection = conn;
                info!("Successfully connected to the database");
                break;
            }
            Err(e) => {
                info!(
                    "Failed to connect to the database: {}. Retrying in 5 seconds...",
                    e
                );
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        }
    }

    run(&address, connection)?.await?;
    Ok(())
}

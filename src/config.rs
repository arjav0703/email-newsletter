use log::{error, info};

#[derive(serde::Deserialize, Debug)]
pub struct Settings {
    pub database_settings: DatabaseSettings,
    pub app_port: u16,
}

#[derive(serde::Deserialize, Debug)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }

    /// Uses the connection string specified in struct to connect PgPool
    pub async fn connect(&self) -> Result<sqlx::PgPool, sqlx::Error> {
        let connection_string = self.connection_string();
        let c = sqlx::PgPool::connect(&connection_string).await?;
        Ok(c)
    }

    pub async fn try_connect(&self) -> Result<sqlx::PgPool, sqlx::Error> {
        let connection;
        loop {
            info!("Attempting to connect to the database...");
            let c = self.connect().await;
            match c {
                Ok(conn) => {
                    connection = conn;
                    info!("Successfully connected to the database");
                    break;
                }
                Err(e) => {
                    error!(
                        "Failed to connect to the database: {}. Retrying in 5 seconds...",
                        e
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                }
            }
        }
        Ok(connection)
    }
    pub async fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}

impl Settings {
    pub async fn from_yaml() -> Result<Self, config::ConfigError> {
        let settings = config::Config::builder()
            .add_source(config::File::new("config.yaml", config::FileFormat::Yaml))
            .build()?;
        settings.try_deserialize::<Settings>()
    }
}

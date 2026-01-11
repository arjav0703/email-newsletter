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
}

impl Settings {
    pub async fn from_yaml() -> Result<Self, config::ConfigError> {
        let settings = config::Config::builder()
            .add_source(config::File::new("config.yaml", config::FileFormat::Yaml))
            .build()?;
        settings.try_deserialize::<Settings>()
    }
}

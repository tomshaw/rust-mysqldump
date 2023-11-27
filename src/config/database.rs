use std::env;
use dotenv::dotenv;
use serde::Deserialize;
use mysql::OptsBuilder;

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub db_host: String,
    pub db_port: u16,
    pub db_username: String,
    pub db_password: String,
    pub db_exports: Vec<String>,
    pub db_forgets: Vec<String>,
    pub db_folder: String,
}

impl DatabaseConfig {
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenv().ok();

        let db_exports: Vec<String> = env::var("DB_EXPORTS")?
            .split(',')
            .map(|s| s.to_string())
            .collect();

        let db_forgets: Vec<String> = env::var("DB_FORGETS")?
            .split(',')
            .map(|s| s.to_string())
            .collect();

        Ok(Self {
            db_host: env::var("DB_HOST")?,
            db_port: env::var("DB_PORT")?.parse::<u16>().map_err(|_| env::VarError::NotPresent)?,
            db_username: env::var("DB_USERNAME")?,
            db_password: env::var("DB_PASSWORD")?,
            db_folder: env::var("DB_FOLDER")?,
            db_exports,
            db_forgets,
        })
    }

    #[allow(dead_code)]
    pub fn mysql_url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}",
            self.db_username, self.db_password, self.db_host, self.db_port
        )
    }

    pub fn mysql_opts(&self) -> OptsBuilder {
        let builder = OptsBuilder::new()
            .ip_or_hostname(Some(&self.db_host))
            .tcp_port(self.db_port)
            .user(Some(&self.db_username))
            .pass(Some(&self.db_password));
        builder
    }
    
}

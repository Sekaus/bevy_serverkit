use postgres::{Client, NoTls};

use serde::Deserialize;

use serde_json::Value;

use std::fs;

#[derive(Debug, Deserialize)]
struct Config {
    database: DatabaseConfig,
}

#[derive(Debug, Deserialize)]
struct DatabaseConfig {
    host: String,
    port: u16,
    user: String,
    password: String,
    dbname: String,
}

pub struct Database {
    client: Client,
}

impl Database {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Read TOML file
        let config_str = fs::read_to_string("config/server.toml")?;

        // Parse TOML
        let config: Config = toml::from_str(&config_str)?;

        // Build connection string
        let connection_string = format!(
            "host={} port={} user={} password={} dbname={}",
            config.database.host,
            config.database.port,
            config.database.user,
            config.database.password,
            config.database.dbname,
        );

        // Connect to PostgreSQL
        let client = Client::connect(&connection_string, NoTls)?;

        println!("Connected to database!");

        Ok(Self { client })
    }

    pub fn init_data_rows(&mut self) -> Result<(), postgres::Error> {
        // Setup player database (if not already)
        self.client.batch_execute(
            r#"
            CREATE TABLE IF NOT EXISTS players (
                id SERIAL PRIMARY KEY,
                name TEXT UNIQUE NOT NULL,
                data JSONB NOT NULL
            )
            "#,
        )?;

        Ok(())
    }

    pub fn insert(&mut self, name: &str, data: &Value) -> Result<(), postgres::Error> {
        self.client.execute(
            r#"
            INSERT INTO players (name, data)
            VALUES ($1, $2)
            ON CONFLICT (name)
            DO UPDATE SET data = EXCLUDED.data
            "#,
            &[&name, data],
        )?;

        Ok(())
    }

    pub fn select(&mut self, name: &str) -> Result<Option<Value>, postgres::Error> {
        let row = self.client.query_opt(
            r#"
                SELECT data
                FROM players
                WHERE name = $1
                "#,
            &[&name],
        )?;

        if let Some(row) = row {
            let data: Value = row.get(0);
            Ok(Some(data))
        } else {
            Ok(None)
        }
    }

    pub fn delete(&mut self, name: &str) -> Result<u64, postgres::Error> {
        let rows_affected = self.client.execute(
            r#"
                DELETE FROM players
                WHERE name = $1
                "#,
            &[&name],
        )?;

        Ok(rows_affected)
    }
}

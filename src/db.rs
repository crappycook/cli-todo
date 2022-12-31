pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::{env, process};

// Getting Started: https://diesel.rs/guides/getting-started
pub struct Config {
    db_url: String,
}

// init db connection
pub fn init_db() -> SqliteConnection {
    let cfg = parse_config().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1)
    });

    establish_connection(&cfg)
}

// parse db config
fn parse_config() -> Result<Config, String> {
    dotenv().ok();

    let database_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(err) => return Err(err.to_string()),
    };

    Ok(Config {
        db_url: database_url.clone(),
    })
}

// connect sqlite
pub fn establish_connection(db_cfg: &Config) -> SqliteConnection {
    SqliteConnection::establish(db_cfg.db_url.as_str()).unwrap_or_else(|err| {
        eprintln!("Connect sqlite error: {err}");
        process::exit(1)
    })
}

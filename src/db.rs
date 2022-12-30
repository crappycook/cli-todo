pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

// Getting Started: https://diesel.rs/guides/getting-started

struct Config {
    db_url: String,
}

fn parse_config() -> Config {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    Config {
        db_url: database_url.clone(),
    }
}

pub fn establish_connection() -> SqliteConnection {
    let cfg = parse_config();
    SqliteConnection::establish(cfg.db_url.as_str())
        .unwrap_or_else(|_| panic!("Error connecting to {}", cfg.db_url))
}

pub mod models;
pub mod schema;

use self::models::*;
use self::schema::*;
use core::str;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::{env, error::Error, process};

// Getting Started: https://diesel.rs/guides/getting-started
pub struct Config {
    db_url: String,
}

// init db connection
pub fn init_db() -> SqliteConnection {
    let cfg = parse_config().unwrap_or_else(|err| {
        eprintln!("Problem parsing configs: {err}");
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

pub fn create_todo_item(
    conn: &mut SqliteConnection,
    title: &str,
    content: &str,
) -> Result<(), Box<dyn Error>> {
    let new_item = NewItem { title, content };

    diesel::insert_into(items::table)
        .values(&new_item)
        .execute(conn)?;

    Ok(())
}

pub fn get_all_item(conn: &mut SqliteConnection) {
    use self::schema::items::dsl::*;

    let records: Vec<Item> = items
        .order(id.desc())
        .limit(10)
        .load::<Item>(conn)
        .expect("Error loading posts");

    for r in records {
        println!("title: {}, content: {}", r.title, r.content);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_insert_item() {
        use super::*;

        let conn = &mut init_db();
        let title = "Play Football";
        let content = "At 15:00 this Friday";

        assert_eq!(create_todo_item(conn, title, content).is_ok(), true);
    }
}

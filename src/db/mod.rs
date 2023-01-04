pub mod models;
pub mod schema;

use self::models::*;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::{env, error, fmt, process};

// Getting Started: https://diesel.rs/guides/getting-started
pub struct Config {
    db_url: String,
}

#[derive(Debug)]
pub struct DeleteNotExistsError;

impl fmt::Display for DeleteNotExistsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "item for delete not exists")
    }
}

impl error::Error for DeleteNotExistsError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

// Init db connection
pub fn init_db() -> SqliteConnection {
    let cfg = parse_config().unwrap_or_else(|err| {
        eprintln!("Problem parsing configs: {err}");
        process::exit(1)
    });

    establish_connection(&cfg)
}

// Parse db config
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

// Connect sqlite
pub fn establish_connection(db_cfg: &Config) -> SqliteConnection {
    SqliteConnection::establish(db_cfg.db_url.as_str()).unwrap_or_else(|err| {
        eprintln!("Connect sqlite error: {err}");
        process::exit(1)
    })
}

pub fn check_table_exist(conn: &mut SqliteConnection) -> bool {
    let sql = "SELECT count(*) as cnt FROM sqlite_master WHERE type='table' AND name='items'";
    let res = diesel::sql_query(sql)
        .load::<QueryTableCount>(conn)
        .unwrap();
    res.len() == 1
}

// Create items table if not exists
pub fn create_table_if_not_exists(conn: &mut SqliteConnection) -> bool {
    let sql = "CREATE TABLE IF NOT EXISTS `items` \
    (`id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT, `title` VARCHAR NOT NULL, `content` TEXT NOT NULL)";
    let res = diesel::sql_query(sql).execute(conn);
    println!("create table result {:?}", res);
    res.is_ok()
}

// Run Test: cargo test
#[cfg(test)]
mod tests {
    #[test]
    // Run specific test: cargo test test_insert_item
    fn test_insert_item() {
        use super::*;

        let conn = &mut init_db();
        let title = "Play Football";
        let content = "At 15:00 this Friday";

        assert!(Item::create_item(conn, title, content).is_ok())
    }

    #[test]
    // Run specific test: cargo test test_update_item
    fn test_update_item() {
        use super::schema::items::dsl::*;
        use super::*;

        let conn = &mut init_db();

        let t = "Do Something";
        let c = "At 20:00 today";
        assert_eq!(Item::create_item(conn, t, c).is_ok(), true);

        let i: models::Item = items
            .order(id.desc())
            .first(conn)
            .unwrap_or_else(|_| panic!("Unable to find item"));
        let new_title = "Sleep";
        let new_content = "At 23:00 today";
        assert!(Item::update_item(conn, i.id, new_title, new_content).is_ok(),);
    }
}

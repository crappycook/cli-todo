pub mod models;
pub mod schema;

use self::models::*;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::{env, error, fmt, fs, process};

// Getting Started: https://diesel.rs/guides/getting-started
pub struct Config {
    db_name: String,
    dir: String,
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

    // Create db dir if not exists.
    fs::create_dir_all(&cfg.dir).expect("Create dir failed!");

    establish_connection(&cfg)
}

// Parse db config
fn parse_config() -> Result<Config, String> {
    dotenv().ok();

    let db_name = match env::var("DATABASE_NAME") {
        Ok(name) => name,
        Err(err) => return Err(err.to_string()),
    };

    let dir = match env::var("DIR") {
        Ok(path) => path,
        Err(err) => return Err(err.to_string()),
    };

    Ok(Config { db_name, dir })
}

// Connect sqlite
pub fn establish_connection(db_cfg: &Config) -> SqliteConnection {
    let db_url = format!("{}/{}", db_cfg.dir, db_cfg.db_name);
    SqliteConnection::establish(db_url.as_str()).unwrap_or_else(|err| {
        eprintln!("Connect sqlite error: {err}");
        process::exit(1)
    })
}

pub fn check_table_exist(conn: &mut SqliteConnection) -> bool {
    let sql = "SELECT count(*) as cnt FROM sqlite_master WHERE type='table' AND name='items'";
    let res = diesel::sql_query(sql)
        .load::<QueryTableCount>(conn)
        .unwrap();
    res.get(0).unwrap().cnt == 1
}

// Create items table if not exists
pub fn create_table_if_not_exists(conn: &mut SqliteConnection) -> bool {
    let sql = "CREATE TABLE IF NOT EXISTS `items` \
    (`id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT, `title` VARCHAR NOT NULL, `content` TEXT NOT NULL)";
    let res = diesel::sql_query(sql).execute(conn);
    res.is_ok()
}

// Run Test: cargo test
#[cfg(test)]
mod tests {

    use diesel::prelude::*;
    fn setup(conn: &mut SqliteConnection) {
        use super::*;
        if !check_table_exist(conn) {
            println!("The storage does not exists");
            if !create_table_if_not_exists(conn) {
                eprintln!("Create todo items table failed!");
                process::exit(1);
            }
            println!("Init the data storage!")
        }
    }

    #[test]
    // Run specific test: cargo test test_update_item
    fn test_update_item() {
        use super::schema::items::dsl::*;
        use super::*;

        let conn = &mut init_db();
        setup(conn);

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

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

pub fn create_item(
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

pub fn update_item(
    conn: &mut SqliteConnection,
    id: i32,
    new_title: &str,
    new_content: &str,
) -> Result<(), Box<dyn Error>> {
    use self::schema::items::dsl::{content, items, title};

    diesel::update(items.find(id))
        .set((title.eq(new_title), content.eq(new_content)))
        .execute(conn)?;

    Ok(())
}

impl Item {
    pub fn get_all_item(conn: &mut SqliteConnection) {
        use self::schema::items::dsl::*;

        let records: Vec<Item> = items
            .order(id.desc())
            .load::<Item>(conn)
            .expect("Error loading item");

        for r in records {
            println!("id: {}, title: {}, content: {}", r.id, r.title, r.content);
        }
    }
}

pub fn get_all_item(conn: &mut SqliteConnection) {
    Item::get_all_item(conn);
}

// Delete item by id
pub fn delete_item_by_id(
    conn: &mut SqliteConnection,
    target: i32,
) -> Result<usize, Box<dyn Error>> {
    use self::schema::items::dsl::*;

    let num_deleted = diesel::delete(items.filter(id.eq(target))).execute(conn)?;
    Ok(num_deleted)
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

// Query all items count
pub fn get_all_count(conn: &mut SqliteConnection) -> Result<i64, Box<dyn Error>> {
    use self::schema::items::dsl::*;
    use diesel::dsl::*;

    let r = items.select(count(id)).first::<i64>(conn)?;
    Ok(r)
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

        assert!(create_item(conn, title, content).is_ok())
    }

    #[test]
    // Run specific test: cargo test test_update_item
    fn test_update_item() {
        use super::schema::items::dsl::*;
        use super::*;

        let conn = &mut init_db();

        let t = "Do Something";
        let c = "At 20:00 today";
        assert_eq!(create_item(conn, t, c).is_ok(), true);

        let i: models::Item = items
            .order(id.desc())
            .first(conn)
            .unwrap_or_else(|_| panic!("Unable to find item"));
        let new_title = "Sleep";
        let new_content = "At 23:00 today";
        assert!(update_item(conn, i.id, new_title, new_content).is_ok(),);
    }
}

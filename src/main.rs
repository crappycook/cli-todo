#[allow(unused_imports)]
#[macro_use]
extern crate diesel;

// extern crate dotenv;

mod db;
use db::*;

fn main() {
    let conn = &mut init_db();

    let exist = check_table_exist(conn);
    println!("table [items] exists: {}", exist);

    println!("items count = {}", get_all_count(conn).unwrap());

    get_all_item(conn);
}

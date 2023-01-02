#[allow(unused_imports)]
#[macro_use]
extern crate diesel;

// extern crate dotenv;

mod cmd;
mod db;

use db::*;
use std::process;

fn main() {
    let conn = &mut init_db();

    if !check_table_exist(conn) {
        println!("our table not exists");
        if !create_table_if_not_exists(conn) {
            eprintln!("Create todo items table failed!");
            process::exit(1);
        }
    }

    println!("items count = {}", get_all_count(conn).unwrap());

    get_all_item(conn);

    cmd::run();
}

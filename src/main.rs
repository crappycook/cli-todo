#[macro_use]
extern crate diesel;

// extern crate dotenv;

mod db;
use db::*;

fn main() {
    init_db();
}

#[allow(unused_imports)]
#[macro_use]
extern crate diesel;

// extern crate dotenv;

mod db;
use db::*;

fn main() {
    let conn =&mut init_db();

    let title = "Having Dinner";
    let content = "At 20:00 today";
    create_todo_item(conn, title, content).expect("create todo item failed");
    
    get_all_item(conn);
}

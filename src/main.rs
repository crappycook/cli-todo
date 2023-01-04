#[allow(unused_imports)]
#[macro_use]
extern crate diesel;

// extern crate dotenv;

mod cmd;
mod db;

fn main() {
    cmd::run();
}

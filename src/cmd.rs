use crate::db::{models::Item, *};
use clap::{Args, Parser, Subcommand};
use std::process;

#[derive(Parser)]
#[command(author = "oasis")]
#[command(version = "0.1.0")]
#[command(about = "Command line todo-list tool written in Rust", long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add todo item
    Add(Adder),
    /// List all todo items
    List,
    /// Remove todo item by id
    Rm(Remover),
}

#[derive(Args)]
struct Adder {
    /// Item's title
    #[arg(short, long)]
    title: String,
    /// Item's content
    #[arg(short, long)]
    content: Option<String>,
}

#[derive(Args)]
struct Remover {
    /// Give the item's id
    #[arg(short, long)]
    id: i32,
}

pub fn run() {
    let conn = &mut init_db();

    if !check_table_exist(conn) {
        println!("The todo item table not exists");
        if !create_table_if_not_exists(conn) {
            eprintln!("Create todo items table failed!");
            process::exit(1);
        }
    }

    let cli = Cli::parse();

    match &cli.command {
        Commands::Add(h) => {
            let content = h.content.as_deref().unwrap_or("");
            match Item::create_item(conn, &h.title, content) {
                Ok(_) => {
                    println!("Create succeed!");
                }
                Err(err) => {
                    println!("Create error: {}!", err)
                }
            };
        }
        Commands::List => {
            Item::get_all_item(conn);
        }
        Commands::Rm(h) => match Item::delete_item_by_id(conn, h.id) {
            Ok(num) => {
                println!("Delete {num} items succeed!");
            }
            Err(err) => {
                eprintln!("Delete item error: {}!", err)
            }
        },
    }
}

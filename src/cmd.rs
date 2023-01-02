use clap::{Args, Parser, Subcommand};

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
    #[arg(long)]
    title: String,
    /// Item's content
    #[arg(long)]
    content: Option<String>,
}

#[derive(Args)]
struct Remover {
    /// Give the item's id
    #[arg(long)]
    id: i32,
}

pub fn run() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add(h) => {
            println!(
                "get title = {}, content = {:?}",
                h.title,
                h.content.as_deref()
            )
        }
        Commands::List => {
            println!("get the list command!")
        }
        Commands::Rm(h) => {
            println!("get the remove id = {}", h.id)
        }
    }
}

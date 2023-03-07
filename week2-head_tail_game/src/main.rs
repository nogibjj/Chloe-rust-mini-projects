// A command-line tool that adds numbers
use clap::Parser;
use headtail::game;

#[derive(Parser)]
#[clap(
    version = "1.0", 
    author = "Chloe Kang", 
    about = "A head or tail game")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(
        version = "1.0", 
        author = "Chloe Kang",
        about = "A head or tail game")]
    Game {
        #[clap(short, long)]
        coin: String,
    },  
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Game { coin }) => game(coin),
        None => println!("No command was used"),
    }
}
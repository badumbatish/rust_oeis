
use clap::{Parser, Subcommand};

/// Program to analyze integer sequence
#[derive(Parser, Debug)]
#[command(author = "Jasmine Tang", version, about, long_about = None )]
struct CLI {
    /// Name of the person to greet
    #[arg(short, long)]
    query: String,


    #[command(subcommand)]
    command : Commands,
}

#[derive(Debug)]
#[derive(Subcommand)]
enum Commands {
    

   Query { input: Option<String> },
}

fn main() {
    let args = CLI::parse();
    

    match &args.command {
        Commands::Query { input} => {
            println!("{input:?}:")
        }
    }

} 

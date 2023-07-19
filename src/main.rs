use std::collections::HashMap;

use tokio;
use clap::{Parser, Subcommand};
use reqwest;
use serde_json;



/// Program to analyze integer sequence
#[derive(Parser, Debug)]
#[command(author = "Jasmine Tang", version, about, long_about = None)]
struct CLI {


    #[command(subcommand)]
    command : Commands,
}

#[derive(Debug)]
#[derive(Subcommand)]
enum Commands {
    
    /// Query a sequence of number from OEIS
    Query { input: Option<String> },
}


#[tokio::main]
async fn main() -> Result<(), Box< dyn std::error::Error>> {
    let args = CLI::parse();
    
    match &args.command {
        Commands::Query { input} => {
            let res = reqwest::get("https://oeis.org/search?q={input}&fmt=json")
                        .await?
                        .json::<serde_json::Value>()
                        .await?;
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        }
    };
    
    Ok(())
} 

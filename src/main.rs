extern crate clap;
use clap::{Parser,Subcommand};
mod head;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    // /// provide numbers in the following format: 'x y z ...'
    // #[clap(short = '\u{006E}', long = "numbers")]
    // nums: String,
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Bubble {
        nums: String },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Bubble { nums } => {

            let numbers: Vec<i32> = nums
                .split_whitespace()
                .map(|s| s.parse().expect("parse error"))
                .collect();

            let bubble_sorted: Vec<i32> = head::sorts::bubble_sort(&numbers);

            println!("{:?}", bubble_sorted);
        }
    }
}

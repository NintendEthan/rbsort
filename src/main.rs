extern crate clap;
use clap::{Parser,Subcommand};
mod sorts;

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

        let mut numbers: Vec<i32> = nums
            .split_whitespace()
            .map(|s| s.parse().expect("parse error"))
            .collect();

        sorts::bubble_sort(&mut numbers);

        print_vec(&numbers);

        }
    }
}

fn print_vec(vec: &Vec<i32>) {
    for i in vec {
        println!("{}", i);
    }
}

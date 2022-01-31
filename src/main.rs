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
    /// use bubble sort to sort a list of numbers
    Bubble {
        /// provide numbers in the following format: 'x y z ...'
        nums: String
    },
    /// use merge sort to sort a list of numbers
    Merge {
        /// provide numbers in the following format: 'x y z ...'
        nums: String
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Bubble { nums } => {
            let numbers: Vec<i32> = head::nums(&nums);
            let bubble_sorted: Vec<i32> = head::sorts::bubble_sort(&numbers);

            println!("{:?}", bubble_sorted);
        }
        
        Commands::Merge { nums } => {
            let numbers: Vec<i32> = head::nums(&nums);
            let merge_sorted: Vec<i32> = head::sorts::mod_merge::merge_sort(&numbers);

            println!("{:?}", merge_sorted);
        }   
    }
}

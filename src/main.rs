extern crate clap;
use clap::{Parser,Subcommand};
mod head;
use head::nums_from_string;
use head::sorts::{bubble_sort,mod_merge};

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
            let numbers: Vec<i32> = nums_from_string(&nums);
            let bubble_sorted: Vec<i32> = bubble_sort(&numbers);

            println!("{:?}", bubble_sorted);
        }
        
        Commands::Merge { nums } => {
            let numbers: Vec<i32> = nums_from_string(&nums);
            let merge_sorted: Vec<i32> = mod_merge::merge_sort(&numbers);

            println!("{:?}", merge_sorted);
        }   
    }
}

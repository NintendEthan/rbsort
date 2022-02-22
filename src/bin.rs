extern crate clap;
use clap::{Parser,Subcommand};
mod sorts; // include local library
use sorts::{bubble_sort,mod_merge};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
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

fn nums_from_string(nums: &String) -> Vec<i32> {
    let new_nums: Vec<i32> = nums.split_whitespace().map(|s| s.parse().expect("parse error")).collect();
    new_nums
}

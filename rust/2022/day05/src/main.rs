use clap::Parser;
use std::process;

use day05::{self, input::get_file_lines};

#[derive(Parser)]
struct Arguments {
    /// First part's input file name
    #[clap(short, long)]
    first_part: Option<String>,
    /// Second part's input file name
    #[arg(short, long)]
    second_part: Option<String>,
}

fn main() {
    let args = Arguments::parse();

    if let Some(file_name) = &args.first_part {
        let lines = match get_file_lines(file_name) {
            Ok(lines) => lines,
            Err(err) => {
                eprint!("Error reading first part file: {}", err);
                process::exit(1);
            }
        };
        println!("First part: {}", day05::get_top_crates_after_moves_from_top(lines));
    }

    if let Some(file_name) = &args.second_part {
        let lines = match get_file_lines(file_name) {
            Ok(lines) => lines,
            Err(err) => {
                eprint!("Error reading second part file: {}", err);
                process::exit(1);
            }
        };
        println!("Second part: {}", day05::get_top_crates_after_moves_from_bottom(lines));
    }
}

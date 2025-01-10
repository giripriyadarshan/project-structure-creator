use clap::Parser;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Optional input file path. If not provided, reads from stdin.
    #[arg(short, long)]
    input: Option<String>,

    /// Optional output directory. If not provided, uses current directory.
    #[arg(short, long, default_value = ".")]
    output_dir: String,
}

fn main() {
    let args = Args::parse();

    let input: Box<dyn BufRead> = if let Some(input_file) = args.input {
        Box::new(io::BufReader::new(fs::File::open(input_file).unwrap()))
    } else {
        println!("Please paste the file structure below.");
        Box::new(io::BufReader::new(io::stdin()))
    };

    let mut current_path = Vec::new();
    let mut last_depth = 0;

    for line in input.lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            continue;
        }

        let depth = line.chars().filter(|&c| c == '│').count()
            + line.chars().filter(|&c| c == '├').count()
            + line.chars().filter(|&c| c == '└').count();

        let clean_line = line
            .replace("├", "")
            .replace("└", "")
            .replace("─", "")
            .replace("│", "")
            .trim()
            .to_string();

        if depth <= last_depth {
            while current_path.len() > depth {
                current_path.pop();
            }
        }

        current_path.push(clean_line.clone());
        last_depth = depth;

        let full_path = Path::new(&args.output_dir).join(current_path.join("/"));

        if clean_line.ends_with('/') {
            if !full_path.exists() {
                fs::create_dir_all(&full_path).unwrap();
                println!("Created directory: {}", full_path.display());
            }
        } else if !full_path.exists() {
            if let Some(parent) = full_path.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            fs::File::create(&full_path).unwrap();
            println!("Created file: {}", full_path.display());
        }
    }
}

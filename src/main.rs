use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::io::{self, BufRead};
use std::path::PathBuf;

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

struct FileTreeProcessor {
    output_dir: PathBuf,
    current_path: Vec<String>,
    last_depth: usize,
    indent_count: usize,
}

impl FileTreeProcessor {
    fn new(output_dir: PathBuf) -> Self {
        Self {
            output_dir,
            current_path: Vec::new(),
            last_depth: 0,
            indent_count: 0,
        }
    }

    fn process_line(&mut self, line: &str) -> Result<()> {
        if line.trim().is_empty() {
            return Ok(());
        }

        let depth = self.calculate_depth(line);
        let clean_line = self.clean_line(line);
        self.update_current_path(depth, clean_line.clone());

        let full_path = self.output_dir.join(self.current_path.join("/"));
        self.create_filesystem_entry(&full_path, clean_line.ends_with('/'))?;

        Ok(())
    }

    fn calculate_depth(&mut self, line: &str) -> usize {
        let mut count = line
            .chars()
            .filter(|&c| matches!(c, '│' | '├' | '└'))
            .count();

        if count == 2 && self.indent_count == 0 {
            let indent_count = line
                .chars()
                .map(|c| if c == ' ' { 1 } else { 0 })
                .sum::<usize>();

            self.indent_count = indent_count;
        }

        if line.starts_with(' ') {
            let indent_count = line
                .chars()
                .map(|c| if c == ' ' { 1 } else { 0 })
                .sum::<usize>();
            count += indent_count % 4;
        }

        count
    }

    fn clean_line(&self, line: &str) -> String {
        line.trim_start_matches(['│', '├', '└', '─'])
            .trim()
            .to_string()
    }

    fn update_current_path(&mut self, depth: usize, clean_line: String) {
        if depth <= self.last_depth {
            while self.current_path.len() > depth {
                self.current_path.pop();
            }
        }
        self.current_path.push(clean_line);
        self.last_depth = depth;
    }

    fn create_filesystem_entry(&self, path: &PathBuf, is_directory: bool) -> Result<()> {
        if path.exists() {
            return Ok(());
        }

        if is_directory {
            fs::create_dir_all(path)
                .with_context(|| format!("Failed to create directory: {}", path.display()))?;
            println!("Created directory: {}", path.display());
        } else {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).with_context(|| {
                    format!("Failed to create parent directory: {}", parent.display())
                })?;
            }
            fs::File::create(path)
                .with_context(|| format!("Failed to create file: {}", path.display()))?;
            println!("Created file: {}", path.display());
        }
        Ok(())
    }
}

fn get_input_reader(input_path: Option<String>) -> Result<Box<dyn BufRead>> {
    match input_path {
        Some(path) => {
            let file = fs::File::open(&path)
                .with_context(|| format!("Failed to open input file: {}", path))?;
            Ok(Box::new(io::BufReader::new(file)))
        }
        None => {
            println!("Please paste the file structure below.");
            Ok(Box::new(io::BufReader::new(io::stdin())))
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let input = get_input_reader(args.input)?;
    let mut processor = FileTreeProcessor::new(PathBuf::from(args.output_dir));

    for line in input.lines() {
        let line = line.with_context(|| "Failed to read line from input")?;
        processor.process_line(&line)?;
    }

    Ok(())
}

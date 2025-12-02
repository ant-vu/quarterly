use clap::Parser;

/// Quarterly report tooling (example CLI)
#[derive(Parser)]
#[command(author = "ant-vu", version = "0.1.0", about = "Quarterly report tooling", long_about = None)]
struct Cli {
    /// Input data file to generate a report from
    #[arg(short, long)]
    input: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    println!("quarterly v0.1.0");
    match cli.input {
        Some(path) => match quarterly::count_lines_in_file(&path) {
            Ok(count) => println!("Found {} non-empty lines in {}", count, path),
            Err(e) => eprintln!("Error reading {}: {}", path, e),
        },
        None => println!("Run with --input <file> to generate a report."),
    }
}

use clap::Parser;
use anyhow::{Context, Result};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    //let args = Cli::parse();
    //println!("pattern : {}\npath: {}", args.pattern, args.path.display());

    let path = "C:\\Users\\aguir\\OneDrive\\Bureau\\copypasta_OW.txt";
    println!("{}", path);
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("could not read file `{}`", path))?;
    // the question mark return an error if the 'Result' enum before isn't a string
    // map_err provide a custom error display (nicer and more readable)

    println!("File content : {}", content);

    Ok(()) // return a default 'Result<(), Box<dyn std::error::Error>>' value

}
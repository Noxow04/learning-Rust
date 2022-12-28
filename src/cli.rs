// cargo run --bin cli test test.txt

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
    let args = Cli::parse();
    let Cli { path, pattern} = args;
    let path: &str= &path.into_os_string().into_string().unwrap();

    let content = std::fs::read_to_string(path)
        .with_context(|| format!("could not read file `{}`", path))?;
    // the question mark return an error if the 'Result' enum before isn't a string
    // with_context provide a nicer error display

    for line in content.lines() {
        if line.contains(&pattern) {
            println!("{}", line);
        }
    }

    Ok(()) // return a default 'Result<()>' value
}

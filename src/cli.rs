use anyhow::{Context, Result};

const COMMAND_LIST: &str = "grep\nln";

fn main() -> Result<()> {
    let command = std::env::args().nth(1).unwrap();
    match &command[..] {    // using splicing to convert String to str
        "help" => {
            let parameter = std::env::args().nth(2);
            match parameter {
                None => println!("Display the function of a specified command and its usage\nUsage : help <command>\nList of commands :\n{}", COMMAND_LIST),
                Some(command) => {
                    match &command[..] {
                        "grep" => println!("Display each line in the target file containing the specified pattern.\nUsage : grep <pattern> <path>"),
                        "ln" => println!("Display each file and directory inside the specified directory (no path provided will display the files and directory inside the current one)\nUsage : ln <path>"),
                        _ => println!("Unknown command"),
                    }
                },
            }
        },
        "grep" => {
            let pattern = std::env::args().nth(2).unwrap();
            let path = std::env::args().nth(3).unwrap();
            return grep(pattern, path, &mut std::io::stdout());
        },
        "ln" => {
            let parameter = std::env::args().nth(2);
            return match parameter {
                None => ln("", &mut std::io::stdout()),
                Some(path) => ln(&path[..], &mut std::io::stdout())
            };
        },
        _ => println!("Unknown command"),
    }

    Ok(())      // Default Result<()> value to return
}


/// function that get the content from a file whose path is specified and apply the "find_matches" function to it
fn grep(pattern: String, path_to_file: String, writer: impl std::io::Write) -> Result<()> {
    let content = std::fs::read_to_string(&path_to_file)           // read the content of the file and store it inside the 'content' variable
        .with_context(|| format!("could not read file `{}`", path_to_file))?;   // returning an error in case of a wrong path
    find_matches(&content, &pattern, writer);
    Ok(())      // Default Result<()> value to return
}

/// function that find and write (using the 'writer' parameter) each line containing the 'pattern' inside the 'content'
fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line).unwrap();
        }
    }
}

/// function that get and write (using the 'writer' parameter) the files and directories inside a directory whose path is specified
fn ln(path_to_directory: &str, mut writer: impl std::io::Write) -> Result<()>{
    let files = std::fs::read_dir(path_to_directory).unwrap();
    for file in files {
        writeln!(writer, "{}", file.unwrap().path().display())?;
    }
    Ok(())   // Default Result<()> value to return
}


#[cfg(test)]
mod tests {
    use std::fs::File;
    use super::*;
    use assert_cmd::prelude::*; // Add methods on commands
    use assert_fs::prelude::*;
    use predicates::prelude::*; // Used for writing assertions
    use std::process::Command;  // Run programs
    use tempdir::TempDir;

    
    /// testing function to verify correct implementation of the "find_matches" function
    #[test]
    fn find_matches_in_content() {
        let mut result: Vec<u8> = Vec::new();
        find_matches("test\nlorem ipsum\ntest 2\ntes\ntest3", "test", &mut result);
        assert_eq!(result, b"test\ntest 2\ntest3\n");       // the b prefix convert the string into a byte string (an array of ASCII characters), which is what the result Vector is
    }

    /// testing function to verify correct implementation of the "ln" function
    #[test]
    fn find_file_in_directory() -> Result<(), Box<dyn std::error::Error>> {
        let temporary_directory = TempDir::new("tests")?;
        let _temporary_file = File::create(temporary_directory.path().join("test-file.txt"))?;

        let mut cmd = Command::cargo_bin("cli")?;
        cmd.arg("ln").arg(temporary_directory.path());
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("test-file.txt"));
        Ok(())
    }

    /// testing function to verify correct implementation of the "grep" function
    #[test]
    fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
        let file = assert_fs::NamedTempFile::new("sample.txt")?;    // creating a temporary file to avoid unnecessary files
        file.write_str("A test\nActual content\nMore content\nAnother test")?;

        let mut cmd = Command::cargo_bin("cli")?;
        cmd.arg("grep").arg("test").arg(file.path());
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("test\nAnother test"));
        Ok(())
       }

    #[test]
    fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("cli")?;
        cmd.arg("grep").arg("test").arg("test/file/doesnt/exist");
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("could not read file"));
        Ok(())
    }
}

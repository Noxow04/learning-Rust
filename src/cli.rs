use anyhow::{Context, Result};

const COMMAND_LIST: &str = "grep";

fn main() -> Result<()> {
    let command = std::env::args().nth(1).unwrap();
    match &command[..] {
        "help" => {
            let parameter = std::env::args().nth(2);
            match parameter {
                None => println!("List of commands :\n{}", COMMAND_LIST),
                Some(command) => {
                    match &command[..] {
                        "grep" => println!("Show each line in the target file containing \
                        the specified pattern.\nUsage : grep <pattern> <path>"),
                        _ => println!("Unknown command"),
                    }
                },
            }
        },
        "grep" => {
            let pattern = std::env::args().nth(2).unwrap();
            let path = std::env::args().nth(3).unwrap();
            let _ = grep(pattern, path, &mut std::io::stdout());
        },
        _ => println!("Unknown command"),
    }

    Ok(())
}

fn grep(pattern: String, path: String, writer: impl std::io::Write) -> Result<()> {
    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("could not read file `{}`", path))?;
    find_matches(&content, &pattern, writer);
    Ok(())
}
fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            let _ = writeln!(writer, "{}", line);
        }
    }
}
#[test]
fn check_grep() {
    let mut result = Vec::new();
    let _ = grep("test".to_string(), "tests/cli.txt".to_string(), &mut result);
    assert_eq!(result,b"test 1\ntest 2\ntest 3\nceci est un test\nbonjour (test)\n");
}
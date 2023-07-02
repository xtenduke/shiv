use std::process::{Command, Stdio, Output};
use std::io::{Write};

/// Run command in dir by spawning bash and sending commands through stdin
pub fn run_on_shell(command: &String, dir: &String) -> Result<Output, std::io::Error> {

    // create new stream for stdout and stderr
    // read line, prefix with [package] so logs can be read

    let mut child = Command::new("bash").current_dir(&dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let child_stdin = child.stdin.as_mut().unwrap();
    child_stdin.write_all(&command.as_bytes())?;
    child_stdin.write_all(b"\n")?;

    let res = child.wait_with_output();
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::test_support::TEST_DATA_DIR;

    #[test]
    fn run_on_shell_can_run_ls() {
        let result = run_on_shell(&String::from("ls"), &String::from(TEST_DATA_DIR)).unwrap();

        let stdout_string = String::from_utf8(result.stdout);
        assert_eq!(stdout_string.unwrap(), "packages\ntest\n");

        let empty_vec: Vec<u8> = Vec::new();
        assert_eq!(result.stderr, empty_vec);
    }
}
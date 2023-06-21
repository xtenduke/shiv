use std::process::{Command, Stdio, Output};
use std::io::{Write};

/// Run command in dir by spawning bash and sending commands through stdin
pub fn run_on_shell(command: &String, dir: &String) -> Result<Output, std::io::Error> {
    let mut child = Command::new("bash").current_dir(&dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    let child_stdin = child.stdin.as_mut().unwrap();
    child_stdin.write_all(&command.as_bytes())?;
    child_stdin.write_all(b"\n")?;
    drop(child_stdin);
    
    let res = child.wait_with_output();
    return res;
}
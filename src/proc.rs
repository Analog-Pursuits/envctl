use std::error::Error;
use std::process::{Command, Output, Stdio};

#[flame]
pub fn cmd(command: &str) -> Result<Output, Box<dyn Error>> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", command])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .expect("failed to execute process")
    };
    return Ok(output);
}

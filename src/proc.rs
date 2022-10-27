use std::process::{Command, Output};
use std::error::Error;

pub fn cmd() -> Result<Output, Box<dyn Error>> {
  let output = if cfg!(target_os = "windows") {
    Command::new("cmd")
            .args(["/C", "echo hello"])
            .output()
            .expect("failed to execute process")
  } else {
    Command::new("sh")
            .arg("-c")
            .arg("echo hello")
            .output()
            .expect("failed to execute process")
  };
  return Ok(output);
}
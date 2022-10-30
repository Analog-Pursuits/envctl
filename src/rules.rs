use std::error::Error;
use crate::proc::cmd;

pub struct Rules {}

impl Rules {
  pub fn new() -> Rules {
    println!("New Rule engine");
    return Rules{};
  }
  pub fn add(&mut self) -> Result<Self, Box<dyn Error>> {
    println!("Add a Rule");
    return Ok(Self{})
  }
  pub fn update(&mut self) -> Result<Self, Box<dyn Error>> {
    println!("Update a Rule");
    return Ok(Self{})
  }
  pub fn delete(&mut self) -> Result<Self, Box<dyn Error>> {
    println!("Delete a Rule");
    return Ok(Self{})
  }
  pub fn exec(&mut self, input_command: &str) -> Result<Self, Box<dyn Error>> {
    println!("Executing CMD");
    let c = cmd(input_command).unwrap();
    let output = std::str::from_utf8(&  c.stdout);
    println!("Status: {:?}", c.status);
    print!("Stdout: {}", output.unwrap());
    return Ok(Self{})
  }
}
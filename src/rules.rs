use std::error::Error;
use crate::proc::cmd;

pub struct Rules {

}

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
  pub fn exec(&mut self) -> Result<Self, Box<dyn Error>> {
    println!("Executing CMD");
    let c = cmd().unwrap();
    println!("{:?}", c.status);
    return Ok(Self{})
  }
}
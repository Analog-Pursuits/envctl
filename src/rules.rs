use std::{error::Error, fs::read_to_string, fs::File, fs::metadata, fs::read};
use std::io::Read;
use regex::Regex;
use crate::{proc::cmd};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json;
use wasmer::{imports, Instance, Module, Store, Universal};
use wasmer_compiler_cranelift::Cranelift;


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleEngine {
  pub root: Vec<Root>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "PolicyName")]
    pub policy_name: String,
    #[serde(rename = "Matches")]
    pub matches: String,
    #[serde(rename = "Rules")]
    pub rules: Vec<Rule>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rule {
    #[serde(rename = "RuleName")]
    pub rule_name: String,
    pub rulebinary: String,
}

impl Default for RuleEngine {
  fn default() -> Self {
    let default_loc = "envctl.json".to_string();
    let data = read_to_string(default_loc)
        .expect("Unable to read file");

    let config: RuleEngine = serde_json::from_str(&data).unwrap();
    //print!("{:?}", config);
    return config
  }
}

impl RuleEngine {
  #[flame]
  pub fn add(&mut self, _rule_match: String , _rule_file: std::path::PathBuf ) -> std::result::Result<&Self, Box<dyn Error>> {
    //let rule_list = &mut self.root;


    return Ok(self)
  }
  #[flame]
  pub fn update(&mut self) -> std::result::Result<&Self, Box<dyn Error>> {
    println!("Update a Rule");
    return Ok(self)
  }
  #[flame]
  pub fn delete(&mut self) -> std::result::Result<&Self, Box<dyn Error>> {
    println!("Delete a Rule");
    return Ok(self)
  }
  #[flame]
  pub fn exec(&mut self, input_command: &str) -> Result<&Self, Box<dyn Error>> {
    for each in &self.root {
        let re = Regex::new(&each.matches).unwrap();
        if re.is_match(input_command) {
          // implement WASM runtime here
          for rule in &each.rules {
            let binary_location = rule.rulebinary.to_string();
            
            let file_binary = std::fs::read(binary_location).unwrap();
            let compiler = Cranelift::new();
            // Create the store
            let store = Store::new(&Universal::new(compiler).engine());

//            let mut store = Store::new(compiler);

    // Let's compile the Wasm module.
            let module = Module::new(&store, file_binary).unwrap();

    // Create an empty import object.
            let import_object = imports! {};

    println!("Instantiating module...");
    // Let's instantiate the Wasm module.
            let instance = Instance::new(&module, &import_object).unwrap();

            let sum = instance.exports.get_function("main").unwrap();

    println!("Calling `sum` function...");
    // Let's call the `sum` exported function. The parameters are a
    // slice of `Value`s. The results are a boxed slice of `Value`s.
            let results = sum.call(&[]).unwrap();

            println!("Results: {:?}", results);
            return Ok(self)
          }
        } else {
          // no match, passthrough command
          cmd(input_command).unwrap();
          return Ok(self)
        };
      };
      return Ok(self)
  }
}
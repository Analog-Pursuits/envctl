use std::{error::Error, fs::read_to_string};
use directories::BaseDirs;
use regex::Regex;
use crate::{proc::cmd};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json;
use wasmer::{imports, Instance, Module, Store, Value};
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
    let config_dir = BaseDirs::new();
    
    match config_dir {
      Some(x) => {
        let project_conf_dir = x.home_dir();
        let conf_file = project_conf_dir.to_str().unwrap();
        let config_file = conf_file.to_owned() + "/.config/envctl/config.json";
        let data = read_to_string(config_file).expect("Unable to read file");

        let config: RuleEngine = serde_json::from_str(&data).unwrap();
        return config

      },
      None => {
        panic!("Failed to find config file");
      },
    }
    
  }
}

impl RuleEngine {
  //#[flame]
  //pub fn add(&mut self, _rule_match: String , _rule_file: std::path::PathBuf ) -> std::result::Result<&Self, Box<dyn Error>> {
  //  //let rule_list = &mut self.root;


  //  return Ok(self)
  //}

  //#[flame]
  //pub fn update(&mut self) -> std::result::Result<&Self, Box<dyn Error>> {
  //  println!("Update a Rule");
  //  return Ok(self)
  //}

  //#[flame]
  //pub fn delete(&mut self) -> std::result::Result<&Self, Box<dyn Error>> {
  //  println!("Delete a Rule");
  //  return Ok(self)
  //}

  #[flame]
  pub fn exec(&mut self, input_command: &str) -> Result<&Self, Box<dyn Error>> {
    for each in &self.root {
        let re = Regex::new(&each.matches).unwrap();
        if re.is_match(input_command) {
          for rule in &each.rules {
            let binary_location = rule.rulebinary.to_string();
            let file_binary = std::fs::read(binary_location).unwrap();

            let compiler = Cranelift::new();

            let mut store = Store::new(compiler);
            
            let module = Module::new(&store, file_binary).unwrap();
            //let func_type = FunctionType::new(vec![Type::I32], vec![]);
            //let print_str = Function::new_typed_with_env(
            //  &mut store,
            //  
            //  wasm::print_str,
            //);
            //let return_bool = Function::new_typed(
            //  &mut store,
            //  wasm::return_bool,
            //);

            let import_object = imports! {
              //"env" => {
                //// name        // the func! macro autodetects the signature
                //"print" => print_str,
                //"return_bool" => return_bool,
            //},
            
            };
            let instance = Instance::new(&mut store, &module, &import_object)?;
            if instance.exports.contains("envctlMain") {

            let main_func = instance.exports.get_function("envctlMain")?;
            let _results: Box<[Value]> = main_func.call(&mut store, &[]).unwrap();

            cmd(input_command).unwrap();
            return Ok(self)
            } else {
              println!("Main function doesn't exist, or something else is going wrong")
            }
          };
          
        } else {
          // no match, passthrough command
          cmd(input_command).unwrap();
          return Ok(self)
        };
      };
      return Ok(self);
    }
}

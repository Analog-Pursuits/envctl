use std::{fs::read_to_string};
use regex::Regex;
use crate::wasm::run_wasm;
use crate::{directory, proc::cmd};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json;


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
    let data = read_to_string(&directory::get_config_file_location()).expect("Unable to read file");
    let config: RuleEngine = serde_json::from_str(&data).unwrap();
    return config;
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
  pub fn init(&mut self) -> std::result::Result<&Self, &'static str> {
    directory::init_config_file();
    return Ok(self);
  }

  #[flame]
  pub fn exec(&mut self, input_command: &str) -> Result<&Self, &'static str> {
    for each in &self.root {
        let re = Regex::new(&each.matches).unwrap();
        if re.is_match(input_command) {
          for rule in &each.rules {
            let binary_location = rule.rulebinary.to_string();
            let exec = run_wasm(binary_location, &[]);
            match exec {
              Ok(_x) => {
                cmd(input_command).unwrap();
                return Ok(self);
              },
              Err(err) => {
                return Err(err);
              },
            }
          };
          
        } else {
          // no match, passthrough command
          cmd(input_command).unwrap();
          return Ok(self)
        };
      };
      if &self.root.len() == &0 {
          // no rules, policies, or matches, passthrough command
          cmd(input_command).unwrap();
      }
      return Ok(self);
    }
}

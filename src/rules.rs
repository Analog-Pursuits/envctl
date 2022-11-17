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
    pub rule_name: String,
    pub rule_binary: String,
    pub rule_entrypoint: Option<String>,
}

impl Default for RuleEngine {
  fn default() -> Self {
    let data = read_to_string(&directory::get_config_file_location()).expect("Unable to read file");
    let config: RuleEngine = serde_json::from_str(&data).unwrap();
    return config;
  }
}

impl RuleEngine {

  pub fn init(&mut self) -> std::result::Result<&Self, String> {
    directory::init_config_file();
    return Ok(self);
  }

  #[flame]
  pub fn exec(&mut self, input_command: &str) -> Result<&Self, String> {
    for each in &self.root {
        let re = Regex::new(&each.matches).unwrap();
        if re.is_match(input_command) {
          for rule in &each.rules {
            let binary_location = rule.rule_binary.to_string();
                let exec = run_wasm(binary_location, Some(""));
                match exec {
                  Err(err) => {
                    return Err(err.to_string());
                  },
                  _ => {}
            };
          };
          println!("WASM");
          cmd(input_command).unwrap();
          return Ok(self);
        } else {
        };
      };
      // no rules, policies, or matches, passthrough command
      println!("naht WASM");
      cmd(input_command).unwrap();
      return Ok(self);
    }
}

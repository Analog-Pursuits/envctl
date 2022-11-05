extern crate flame;
#[macro_use] extern crate flamer;
use clap::{Subcommand, Parser};
use crate::settings::Settings;
use crate::rules::RuleEngine;

mod settings;
mod wasm;
mod rules;
mod proc;
mod directory;

#[derive(Parser, Debug)]
struct Args {
  #[arg(
    short,
    long,
    global = true,
    required = false,
    default_value_t = false,
  )]
  debug: bool,
  #[command(subcommand)]
  action: Action,
}
#[derive(Debug, Subcommand)]
enum Action {
  //Add {
  //  #[arg()]
  //  rule_match: String,
  //  #[arg()]
  //  rule_file: std::path::PathBuf,
  //},
  //Update,
  //Delete,
  #[command(arg_required_else_help = true )]
  Exec { 
    #[arg(last = true)]
    ext_command: String,
  },
  Init {},
}

#[flame]
fn main() {
  Settings::new().unwrap();
  let args = Args::parse();

  let mut rule_engine: RuleEngine;

  // If not init action then load the config file
  match args.action {
      Action::Init {} => {
          rule_engine = RuleEngine { root: vec![] };
      }
      _ => {
          rule_engine = RuleEngine::default();
      }
  }

  match args.action {
      //Action::Add { rule_match, rule_file } => RuleEngine::add(&mut rule_engine, rule_match, rule_file).unwrap(),
      //Action::Update => RuleEngine::update(&mut rule_engine).unwrap(),
      //Action::Delete => RuleEngine::delete(&mut rule_engine).unwrap(),
      Action::Exec { ext_command } => RuleEngine::exec(&mut rule_engine, &ext_command).unwrap(),
      Action::Init {} => return RuleEngine::init().unwrap(),
  };

  if args.debug == true {
    println!("Outputting Flame graph!");
    flame::dump_html(std::fs::File::create("flamegraph.html").unwrap()).unwrap();
  }
}


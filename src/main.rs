use clap::{Subcommand, Parser};
use crate::settings::Settings;
use crate::rules::Rules;

mod settings;
mod rules;
mod proc;

#[derive(Parser, Debug)]
struct Args {
  #[command(subcommand)]
  action: Action,
}
#[derive(Debug, Subcommand)]
enum Action {
  Add,
  Update,
  Delete,
  #[command(arg_required_else_help = true )]
  Exec { 
    #[arg()]
    ext_command: String,
  }
}

fn main() {

  let _settings = Settings::new();
  let args = Args::parse();
  let mut rule_engine = Rules::new();

  match args.action {
    Action::Add => Rules::add(&mut rule_engine).unwrap(),
    Action::Update => Rules::update(&mut rule_engine).unwrap(),
    Action::Delete => Rules::delete(&mut rule_engine).unwrap(),
    Action::Exec { ext_command } => Rules::exec(&mut rule_engine, &ext_command ).unwrap(),
  };
}


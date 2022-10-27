use clap::Parser;
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
#[derive(Debug, clap::Subcommand)]
enum Action {
  Add,
  Update,
  Delete,
  Exec
}

fn main() {

  let settings = Settings::new();
  let args = Args::parse();
  let mut rule_engine = Rules::new();

  match args.action {
    Action::Add => Rules::add(&mut rule_engine).unwrap(),
    Action::Update => Rules::update(&mut rule_engine).unwrap(),
    Action::Delete => Rules::delete(&mut rule_engine).unwrap(),
    Action::Exec => Rules::exec(&mut rule_engine).unwrap(),

  };
  println!("{:?}", args);
  println!("Settings: {:?}",
    settings
  );

}


use clap::Parser;
use crate::settings::Settings;

mod settings;

#[derive(Parser, Debug)]
struct Cli {
  pattern: String,
  path: std::path::PathBuf,
}

fn main() {

  let args = Cli::parse();

  let settings = Settings::new();
  println!("{:?}", args);
  println!("Settings: {:?}",
    settings
  );
}


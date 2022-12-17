extern crate flame;
#[macro_use]
extern crate flamer;
use crate::rules::RuleEngine;
use crate::settings::Settings;
use clap::{Parser, Subcommand};

#[cfg(test)]
mod tests;

mod directory;
mod proc;
mod rules;
mod settings;
mod wasm;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, global = true, required = false, default_value_t = false)]
    debug: bool,
    #[command(subcommand)]
    action: Action,
}
#[derive(Debug, Subcommand)]
enum Action {
    #[command(arg_required_else_help = true)]
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

    let command = match args.action {
        Action::Exec { ext_command } => RuleEngine::exec(&mut rule_engine, &ext_command),
        Action::Init {} => RuleEngine::init(&mut rule_engine),
    };

    match command {
        Ok(_x) => {}
        Err(err) => {
            println!("{:?}", err)
        }
    }

    if args.debug == true {
        println!("Outputting Flame graph!");
        flame::dump_html(std::fs::File::create("flamegraph.html").unwrap()).unwrap();
    }
}

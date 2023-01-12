use std::env;
use std::fs;

fn main() {
  print!("
    _____     _ _        _ _ _         _   _ 
    |  |  |___| | |___   | | | |___ ___| |_| |
    |     | -_| | | . |  | | | | . |  _| | . |
    |__|__|___|_|_|___|  |_____|___|_| |_|___|
                                              \n");
    for argument in env::args() {
      println!("argument: {argument}");
    };
   // for env in env::vars() {
   //   println!("EnvVar: {:#?}", env);
   // }
    //let config = get_kube_config();
//    match config {
//      Ok(_) => {
//        return;
//      }
//      Err(err) => {
//        panic!("{err}")
//      }
//    }
}

fn get_kube_config() -> Result<(),String> {
  let config = fs::read_to_string("~/.kube/config").unwrap();
  for line in config.lines() {
    if line.contains("current-context: rancher-desktop") {
      return Err("Current context is Rancher Desktop, please use Docker".to_string());
    } else {
      return Ok(());
    };
  }
  return Err("Could not find kube config file".to_string());
}
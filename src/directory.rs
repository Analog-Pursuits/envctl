use directories::BaseDirs;
use std::{fs, path::Path};

pub fn get_config_file_location() -> String {
  return get_config_file_directory() + "config.json";
}

fn get_config_file_directory() -> String {
  let config_dir = BaseDirs::new();

  match config_dir {
      Some(x) => {
          let project_conf_dir = x.home_dir();
          let conf_file = project_conf_dir.to_str().unwrap();
          return conf_file.to_owned() + "/.config/envctl/";
      }
      None => {
          panic!("Failed to find config file");
      }
  };
}

pub fn init_config_file() {
  let file_directory_string = get_config_file_directory();
  let file_directory = Path::new(file_directory_string.as_str());
  let file_path = get_config_file_location();
  if Path::exists(Path::new(&file_path)) {
      panic!("Config file already exists");
  } else {
      fs::create_dir_all(file_directory.parent().unwrap()).unwrap();

      fs::write(&file_path, b"{\"root\":[]}").unwrap();
      println!("Created config file at: {}", file_path);
  }
}

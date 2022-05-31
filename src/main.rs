use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use std::fs;

use crossterm::{
  cursor,
  event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
  terminal::{disable_raw_mode, enable_raw_mode, is_raw_mode_enabled, Clear, ClearType},
};

fn main() {}

fn clear() {}

fn create_file(username: String, password: String) -> bool {
  let mc = new_magic_crypt!(&password, 256);

  let base64 = mc.encrypt_str_to_base64(format!(
    r"
[information]
username = {}

[passwords]
welcome = welcome
  ",
    username
  ));

  let filename = format!("data_{}.simpass", username);

  match fs::write(&filename, base64) {
    Err(..) => false,
    Ok(..) => true,
  }
}



fn load_file(username: String, password: String) -> Result<String, String> {
  let mc = new_magic_crypt!(&password, 256);
  let filename = format!("data_{}.simpass", &username);

  let file_info = fs::metadata(&filename).is_ok();

  if !file_info {
    Err("File not found".to_string())
  } else {
    match fs::read_to_string(filename) {
      Err(..) => Err("Something went wrong while reading data from disk".to_string()),
      Ok(c) => match mc.decrypt_base64_to_string(c) {
        Ok(d) => Ok(d),
        Err(..) => Err("Wrong password".to_string()),
      },
    }
  }
}

fn with_raw_mode_off(f: fn()) {
  let was = is_raw_mode_enabled().unwrap();
  disable_raw_mode().unwrap();
  f();
  if was {
    enable_raw_mode().unwrap();
  }
}

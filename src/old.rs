use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use std::{env, fs};

use crossterm::{
  cursor,
  event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
  terminal::{disable_raw_mode, enable_raw_mode, is_raw_mode_enabled, Clear, ClearType},
};
use std::io::{stdout, Write};

fn load_file(username: String, password: String) {
  let mc = new_magic_crypt!(&password, 256);

  let filename = format!("data_{}.simpass", username);

  let data_exists = fs::metadata(&filename).is_ok();

  if !data_exists {
    register(username, password);
  }

  let contents =
    fs::read_to_string(filename).expect("Something went wrong while reading data from disk");

  let decrypted = mc.decrypt_base64_to_string(contents);

  let data = match decrypted {
    Ok(..) => decrypted.unwrap(),
    _ => "WRONG-PASSWORD".to_string(),
  };

  let lines: Vec<&str> = data.lines().collect();

  println!("{:?}", lines);
}

fn register(username: String, password: String) {
  let mc = new_magic_crypt!(&password, 256);

  let base64 = mc.encrypt_str_to_base64(format!("PASS-IS-CORRECT\nusername: {}", username));

  let filename = format!("data_{}.simpass", username);

  fs::write(&filename, base64).expect("Something went wrong");
}

fn main() {
  disable_raw_mode().unwrap();

  let mut stdout = stdout();

  println!("\r********** LOGIN / REGISTER **********\n");
  println!("\rIf there is no acc with username you enter program will create one for you\n");

  print!("username: ");
  stdout.flush().unwrap();

  let mut username = String::new();

  enable_raw_mode().unwrap();

  loop {
    match read().unwrap() {
      Event::Key(KeyEvent {
        code: KeyCode::Char(c),
        modifiers: KeyModifiers::NONE,
      }) => {
        username.push(c);
        print!("{}", c);
        stdout.flush().unwrap();
      }
      Event::Key(KeyEvent {
        code: KeyCode::Backspace,
        modifiers: KeyModifiers::NONE,
      }) => {
        if username.len() > 0 {
          username.pop();
          print!("\x08 \x08");
          stdout.flush().unwrap();
        }
      }
      Event::Key(KeyEvent {
        code: KeyCode::Enter,
        modifiers: KeyModifiers::NONE,
      }) => break,
      _ => (),
    }
  }

  println!();

  print!("\rpassword: ");
  stdout.flush().unwrap();

  let mut password = String::new();

  enable_raw_mode().unwrap();

  loop {
    match read().unwrap() {
      Event::Key(KeyEvent {
        code: KeyCode::Char(c),
        modifiers: KeyModifiers::NONE,
      }) => {
        password.push(c);
        print!("*");
        stdout.flush().unwrap();
      }
      Event::Key(KeyEvent {
        code: KeyCode::Backspace,
        modifiers: KeyModifiers::NONE,
      }) => {
        if password.len() > 0 {
          password.pop();
          print!("\x08 \x08");
          stdout.flush().unwrap();
        }
      }
      Event::Key(KeyEvent {
        code: KeyCode::Enter,
        modifiers: KeyModifiers::NONE,
      }) => break,
      _ => (),
    }
  }

  println!("\r");

  println!("{:?}", load_file(username, password));

  disable_raw_mode().unwrap();
}

/*

[confirmation]
name = "asd"

[data]
pass1 = "sus"
pass2 = "bruh"

[passwords]
  [emails]
    [gmail]
      [pass] = "sus"
      [pass] = "bruh"
    [yahoo]
      [pass] = "sus"
      [pass] = "bruh"
    [hotmail]
      [pass] = "sus"
      [pass] = "bruh"

*/

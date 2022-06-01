// kdyz chces vypsat bez noveho radku tak musis udelat
//  print!();
//  stdout.flush().unwrap();

const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &str = "0123456789";

use crossterm::{
  cursor,
  event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, is_raw_mode_enabled, Clear, ClearType},
};
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use std::fs;
use std::io::{stdout, Write};

mod lexer;

pub use lexer::Lexer;

const DEBUG: bool = true;

fn main() {
  let f = fs::read_to_string("Cargo.toml").unwrap();
  if DEBUG {
    println!("{}\n", f);
  }
  let mut p = Lexer::new(f);
  let tokens = p.tokenize();
  if DEBUG {
    for t in tokens {
      println!("{:?}", t);
    }
  }

  // clear();
  // register();
  // disable_raw_mode();
}

fn clear() {
  let was = is_raw_mode_enabled().unwrap();
  disable_raw_mode().unwrap();
  execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
  if was {
    enable_raw_mode();
  }
}

fn print_login_register_menu(item: i8) {
  clear();
  let was = is_raw_mode_enabled().unwrap();
  disable_raw_mode().unwrap();
  let mut i1 = String::new();
  let mut i2 = String::new();

  match item {
    0 => {
      i1 = "*".to_string();
      i2 = " ".to_string();
    }
    1 => {
      i2 = "*".to_string();
      i1 = " ".to_string();
    }
    _ => {
      i1 = " ".to_string();
      i2 = " ".to_string();
    }
  }

  println!("********** LOGIN / REGISTER **********\n");

  println!(
    r"
({}) Login
({}) Register
    ",
    i1, i2
  );

  if was {
    enable_raw_mode();
  }
}

fn login_register_menu() {
  let was = is_raw_mode_enabled().unwrap();

  let items = 2;
  let mut item = -1;

  enable_raw_mode().unwrap();

  loop {
    print_login_register_menu(item);
    match read().unwrap() {
      Event::Key(KeyEvent {
        code: KeyCode::Up,
        modifiers: KeyModifiers::NONE,
      }) => {
        item -= 1;
        if item < 0 {
          item = items - 1;
        }
      }
      Event::Key(KeyEvent {
        code: KeyCode::Down,
        modifiers: KeyModifiers::NONE,
      }) => {
        item += 1;
        if item >= items {
          item = 0;
        }
      }
      Event::Key(KeyEvent {
        code: KeyCode::Enter,
        modifiers: KeyModifiers::NONE,
      }) => match item {
        0 => {}
        _ => {
          break;
        }
      },
      _ => (),
    }
  }

  if was {
    enable_raw_mode();
  }
}

fn create_file(username: String, password: String) -> Result<Option<String>, String> {
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

  let file_info = fs::metadata(&filename).is_ok();

  if file_info {
    Err("User already exists".to_string())
  } else {
    match fs::write(&filename, base64) {
      Err(..) => Err("Cant save data".to_string()),
      Ok(..) => Ok(None),
    }
  }
}

fn register_username_loop() -> String {
  let mut stdout = stdout();

  loop {
    print!("\rUsername: ");
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
          code: KeyCode::Char(c),
          modifiers: KeyModifiers::SHIFT,
        }) => {
          username.push(c.to_ascii_uppercase());
          print!("{}", c.to_ascii_uppercase());
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

    if username.len() == 0 {
      println!("\r\nUsername is empty");
      continue;
    }

    let file_info = fs::metadata(&format!("data_{}.simpass", username)).is_ok();

    if file_info {
      println!("\r\nUser already exists");
      continue;
    }

    return username.to_string();
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

// Register

fn register() {
  clear();
  let was = is_raw_mode_enabled().unwrap();
  disable_raw_mode().unwrap();
  let mut stdout = stdout();

  println!("\r********** REGISTER **********\n");

  let username = register_username_loop();

  loop {
    print!("\n\rPassword: ");
    stdout.flush().unwrap();

    let mut password = String::new();
    let mut password_confirmation = String::new();

    loop {
      match read().unwrap() {
        Event::Key(KeyEvent {
          code: KeyCode::Char(c),
          modifiers: KeyModifiers::NONE,
        }) => {
          password.push(c);
          print!("{}", c);
          stdout.flush().unwrap();
        }
        Event::Key(KeyEvent {
          code: KeyCode::Char(c),
          modifiers: KeyModifiers::SHIFT,
        }) => {
          password.push(c.to_ascii_uppercase());
          print!("{}", c.to_ascii_uppercase());
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

    print!("\n\rPassword confirm: ");
    stdout.flush().unwrap();

    loop {
      match read().unwrap() {
        Event::Key(KeyEvent {
          code: KeyCode::Char(c),
          modifiers: KeyModifiers::NONE,
        }) => {
          password_confirmation.push(c);
          print!("{}", c);
          stdout.flush().unwrap();
        }
        Event::Key(KeyEvent {
          code: KeyCode::Char(c),
          modifiers: KeyModifiers::SHIFT,
        }) => {
          password_confirmation.push(c.to_ascii_uppercase());
          print!("{}", c.to_ascii_uppercase());
          stdout.flush().unwrap();
        }
        Event::Key(KeyEvent {
          code: KeyCode::Backspace,
          modifiers: KeyModifiers::NONE,
        }) => {
          if password_confirmation.len() > 0 {
            password_confirmation.pop();
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

    // Min 1 uppercase, min 1 lowercase, min 1 digit, min 8 chars

    let mut uppercase = false;
    let mut lowercase = false;
    let mut number = false;

    for c in password.chars() {
      if UPPERCASE.contains(c) {
        uppercase = true;
      }
      if LOWERCASE.contains(c) {
        lowercase = true;
      }
      if DIGITS.contains(c) {
        number = true;
      }
    }
    if password.len() < 8 {
      println!("\r\nPassword must be at least 8 characters");
      continue;
    }
    if !uppercase {
      println!("\r\nPassword must contain at least 1 uppercase letter");
      continue;
    }
    if !lowercase {
      println!("\r\nPassword must contain at least 1 lowercase letter");
      continue;
    }
    if !number {
      println!("\r\nPassword must contain at least 1 digit");
      continue;
    }

    if password == password_confirmation {
      clear();
      create_file(username, password).unwrap();
      break;
    } else {
      clear();
      println!(
        "Passwords didnt match\r\n{} {}",
        password, password_confirmation
      );
      continue;
    }
  }

  if was {
    enable_raw_mode().unwrap();
  }
}

// Register end

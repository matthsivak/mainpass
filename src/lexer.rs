const DEBUG: bool = true;

const LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &str = "0123456789";
const SYMBOLS: &str = "[].=\"\\";
const WHITESPACE: &str = " \t\n\r";

#[derive(Clone, Debug)]
enum TokenType {
  Symbol(String),
  Identifier(String),
  Number(i32),
  String(String),
  Comment(String),
}

#[derive(Clone, Debug)]
pub struct Token {
  token_type: TokenType,
  whitespace: String,
}

pub struct Lexer {
  input: String,
  current_char: char,
  pos: usize,
  tokens: Vec<Token>,
  buffer: String,
  whitespace: String,
}

impl Lexer {
  pub fn new(input: String) -> Lexer {
    let i = input.clone();
    Lexer {
      input,
      pos: 0,
      current_char: i.chars().nth(0).unwrap(),
      tokens: vec![],
      buffer: String::new(),
      whitespace: String::new(),
    }
  }

  fn advance(&mut self) {
    self.pos += 1;
    if self.pos >= self.input.len() {
      self.current_char = '\0';
    } else {
      self.current_char = self.input.chars().nth(self.pos).unwrap();
    }
  }

  fn make_string(&mut self) {
    self.advance();
    while self.current_char != '"' {
      self.buffer += &self.current_char.to_string();
      self.advance();
    }
    self.advance();
    if DEBUG {
      println!("str {}", self.buffer);
    }
    self.tokens.push(Token {
      token_type: TokenType::String(self.buffer.clone()),
      whitespace: self.whitespace.clone(),
    });
    self.buffer = String::new();
    self.whitespace = String::new();
  }

  fn make_whitespace(&mut self) {
    while WHITESPACE.contains(self.current_char) {
      self.whitespace += &self.current_char.to_string();
      self.advance();
    }
  }

  fn make_number(&mut self) {
    while DIGITS.contains(self.current_char) {
      self.buffer += &self.current_char.to_string();
      self.advance();
    }
    self.advance();
    if DEBUG {
      println!("num {}", self.buffer)
    };
    self.tokens.push(Token {
      token_type: TokenType::Number(self.buffer.parse().unwrap()),
      whitespace: self.whitespace.clone(),
    });
    self.buffer = String::new();
    self.whitespace = String::new();
  }

  fn make_identifier(&mut self) {
    while LETTERS.contains(self.current_char)
      || DIGITS.contains(self.current_char)
      || self.current_char == '_'
      || self.current_char == '-'
    {
      self.buffer += &self.current_char.to_string();
      self.advance();
    }
    if DEBUG {
      println!("id  {}", self.buffer);
    }
    self.tokens.push(Token {
      token_type: TokenType::Identifier(self.buffer.clone()),
      whitespace: self.whitespace.clone(),
    });
    self.buffer = String::new();
    self.whitespace = String::new();
  }

  fn make_symbol(&mut self) {
    self.buffer += &self.current_char.to_string();
    if DEBUG {
      println!("sym {}", self.buffer);
    }
    self.tokens.push(Token {
      token_type: TokenType::Symbol(self.buffer.clone()),
      whitespace: self.whitespace.clone(),
    });
    self.buffer = String::new();
    self.whitespace = String::new();
    self.advance();
  }

  fn make_comment(&mut self) {
    self.advance();
    while self.current_char != '\n' {
      self.buffer += &self.current_char.to_string();
      self.advance();
    }
    self.buffer = self.buffer.trim_start().to_string();
    if DEBUG {
      println!("com {}", self.buffer);
    }
    self.tokens.push(Token {
      token_type: TokenType::Comment(self.buffer.clone()),
      whitespace: self.whitespace.clone(),
    });
    self.buffer = String::new();
    self.whitespace = String::new();
  }

  pub fn tokenize(&mut self) -> Vec<Token> {
    self.tokens = Vec::new();

    while self.current_char != '\0' {
      let c = self.current_char;

      if c == '#' {
        self.make_comment();
        continue;
      }
      if c == '"' {
        self.make_string();
        continue;
      }
      if WHITESPACE.contains(c) {
        self.make_whitespace();
        continue;
      }
      if DIGITS.contains(c) {
        self.make_number();
        continue;
      }
      if LETTERS.contains(c) {
        self.make_identifier();
        continue;
      }
      if SYMBOLS.contains(c) {
        self.make_symbol();
        continue;
      }
    }

    self.tokens.clone()
  }
}

const LETTERS = "abcdefghijklmnopqrstuvwxyz";
const DIGITS = "0123456789";
const SYMBOLS = "[].=\"\\";
const WHITESPACE = " \t\n\r";

pub enum Data {
  Section(String, Vec<Data>),
  Item(String, String),
}

enum TokenType {
  Symbol(char),
  Identifier(String),
  Number(i32),
  String(String),
}

struct Token {
  token_type: TokenType,
  whitespace: String,
}

struct Lexer {
  input: String,
  current_char: char,
  pos: usize,
  tokens: Vec<Token>,
}

impl Lexer {
  fn new(input: String) -> Lexer {
    Lexer { input, pos: 0 }
  }

  fn advance(&mut self) {
    self.pos += 1;
    if self.pos > self.input.len() {
      self.current_char = '\0';
    } else {
      self.current_char = self.input.chars().nth(self.pos - 1).unwrap();
    }
  }

  pub fn tokenize(&mut self) -> Vec<Token> {
    self.tokens = Vec::new();
  }
}

/*
toml
key = data

[key]
key = data
key = data
key = data


rust
  [
    ["key", "data"]
    ["key",
      [
        ["key", "data"],
        ["key", "data"],
        ["key", "data"],
      ]
    ]
  ]

*/

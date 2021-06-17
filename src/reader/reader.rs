use super::super::reader::ast::*;

pub enum Tok {
  Eof,
  Number(f32),
}

pub struct Lexer<'a> {
  it: std::iter::Peekable<std::str::Chars<'a>>,
}

impl Lexer<'_> {
  pub fn new(code: &str) -> Lexer {
    Lexer {
      it: code.chars().peekable(),
    }
  }

  fn next_char(&mut self) -> Option<char> {
    self.it.next()
  }

  fn peek_char(&mut self) -> Option<&char> {
    self.it.peek()
  }

  fn with_peek_char(&mut self, f: fn(char) -> bool) -> bool {
    match self.peek_char() {
      Some(&chr) => f(chr),
      _ => false,
    }
  }

  fn skip_chars_while(&mut self, pred: fn(char) -> bool) {
    loop {
      match self.peek_char() {
        Some(&chr) => {
          if !pred(chr) {
            break;
          } else {
            self.next_char();
          }
        }
        None => {
          break;
        }
      }
    }
  }

  fn skip_whitespace(&mut self) {
    self.skip_chars_while(|chr| chr.is_whitespace());
  }

  pub fn next_token(&mut self) -> Tok {
    self.skip_whitespace();

    match self.next_char() {
      Some(chr) => {
        let mut is_neg = false;
        let mut is_dec = false;

        if self.with_peek_char(|c| c == '-') {
          is_neg = true;
          self.next_char();
        }

        if self.with_peek_char(|c| c == '.') {
          is_dec = true;
          self.next_char();
        }
      }
      _ => return Tok::Eof,
    }

    Tok::Eof
  }

  fn read() {}
}

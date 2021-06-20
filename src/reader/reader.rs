use super::super::reader::ast::*;

#[derive(Debug, PartialEq)]
pub struct Loc {
  line: i32,
  column: i32,
}

impl Loc {
  pub fn blank() -> Loc {
    Loc { line: 0, column: 0 }
  }
}

#[derive(Debug)]
pub enum Tok {
  Eof(Loc),
  Number(f32, Loc),
  Bool(bool, Loc),
}

impl PartialEq for Tok {
  fn eq(&self, other: &Tok) -> bool {
    match (self, other) {
      (Tok::Eof(_), Tok::Eof(_)) => true,
      (Tok::Number(a, _), Tok::Number(b, _)) => a == b,
      (Tok::Bool(a, _), Tok::Bool(b, _)) => a == b,
      _ => false,
    }
  }
}

pub struct Lexer {
  it: usize,
  pin: usize,
  pin_loc: Loc,
  code: Vec<char>,
  loc: Loc,
}

impl Lexer {
  pub fn new(code: &str) -> Lexer {
    let loc = Loc { line: 1, column: 1 };
    Lexer {
      it: 0,
      pin: 0,
      pin_loc: Loc { ..loc },
      code: code.chars().collect(),
      loc: Loc { ..loc },
    }
  }

  pub fn pin(&mut self) {
    self.pin_loc = Loc { ..self.loc };
    self.pin = self.it;
  }

  pub fn unpin(&mut self) {
    self.it = self.pin;
    self.loc = Loc { ..self.pin_loc };
  }

  pub fn at_eof(&self) -> bool {
    self.it >= self.code.len()
  }

  pub fn get_loc(&self) -> Loc {
    Loc { ..self.loc }
  }

  pub fn current_char_or(&self, or: char) -> char {
    if self.at_eof() {
      return or;
    }
    return self.code[self.it];
  }

  pub fn current_char_def(&self) -> char {
    return self.current_char_or('\0');
  }

  pub fn current_char(&self) -> Option<char> {
    if self.at_eof() {
      None
    } else {
      Some(self.code[self.it])
    }
  }

  fn move_next(&mut self) {
    self.loc.column += 1;
    self.it += 1;
  }

  fn get_then_move(&mut self) -> char {
    let v = self.current_char_def();
    self.move_next();
    return v;
  }

  pub fn skip_whitespace(&mut self) {
    while !self.at_eof() && self.current_char_def().is_whitespace() {
      let chr = self.current_char_def();
      if chr == '\n' {
        self.loc.line += 1;
      } else {
        self.loc.column += 1;
      }
      self.move_next();
    }
  }

  pub fn skip_comments(&mut self) {
    if self.current_char_def() == ';' {
      while !self.at_eof() && self.current_char_def() != '\n' {
        self.move_next();
      }
    }
  }

  pub fn next_token(&mut self) -> Tok {
    self.skip_whitespace();
    self.skip_comments();
    self.skip_whitespace();

    let mut builder = String::new();

    self.pin();
    if self.current_char_def() == '-' {
      builder.push('-');
      self.move_next();
      true
    } else {
      false
    };

    let mut is_dec = if self.current_char_def() == '.' {
      builder.push('.');
      self.move_next();
      true
    } else {
      false
    };

    if self.current_char_def().is_numeric() {
      builder.push(self.get_then_move());
      loop {
        if self.current_char_def() == '.' {
          if is_dec {
            break;
          }
          is_dec = true;
          builder.push(self.get_then_move());
          continue;
        }
        if !self.current_char_def().is_numeric() {
          break;
        }
        builder.push(self.get_then_move());
      }

      return Tok::Number(builder.parse().unwrap(), self.get_loc());
    } else {
      builder.clear();
      self.unpin();
    }

    self.pin();
    if self.current_char_def() == '#' {
      self.move_next();

      if self.current_char_def() == 'f' {
        self.move_next();
        return Tok::Bool(false, self.get_loc());
      } else if self.current_char_def() == 't' {
        self.move_next();
        return Tok::Bool(true, self.get_loc());
      } else {
        self.unpin();
      }
    }

    if self.at_eof() {
      return Tok::Eof(self.get_loc());
    }

    panic!("Unexpected character: '{:?}'", self.current_char_def());
  }
}

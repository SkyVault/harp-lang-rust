use super::super::reader::ast::*;

#[derive(Debug, PartialEq)]
pub struct Loc {
  pub line: i32,
  pub column: i32,
}

impl Loc {
  pub fn blank() -> Loc {
    Loc { line: 0, column: 0 }
  }
}

#[derive(Debug)]
pub enum Tok {
  Eof(Loc),
  Number(f64, Loc),
  Bool(bool, Loc),
  Str(String, Loc),
  Atom(String, Loc), // TODO(Dustin): Create an atom dictionary and only store an atom ID
  OpenParen(Loc),
  CloseParen(Loc),
  OpenBrace(Loc),
  CloseBrace(Loc),
  OpenBracket(Loc),
  CloseBracket(Loc),
  Quote(Loc),
}

impl PartialEq for Tok {
  fn eq(&self, other: &Tok) -> bool {
    match (self, other) {
      (Tok::Eof(_), Tok::Eof(_)) => true,
      (Tok::Number(a, _), Tok::Number(b, _)) => a == b,
      (Tok::Str(a, _), Tok::Str(b, _)) => a == b,
      (Tok::Atom(a, _), Tok::Atom(b, _)) => a == b,
      (Tok::Bool(a, _), Tok::Bool(b, _)) => a == b,
      (Tok::OpenParen(_), Tok::OpenParen(_)) => true,
      (Tok::CloseParen(_), Tok::CloseParen(_)) => true,
      (Tok::OpenBrace(_), Tok::OpenBrace(_)) => true,
      (Tok::CloseBrace(_), Tok::CloseBrace(_)) => true,
      (Tok::OpenBracket(_), Tok::OpenBracket(_)) => true,
      (Tok::CloseBracket(_), Tok::CloseBracket(_)) => true,
      (Tok::Quote(_), Tok::Quote(_)) => true,
      _ => false,
    }
  }
}

pub struct Reader {
  it: usize,
  pin: usize,
  pin_loc: Loc,
  code: Vec<char>,
  loc: Loc,
}

pub fn is_delim(chr: char) -> bool {
  match chr {
    '(' | ')' | '[' | ']' | '{' | '}' | '\'' | '\"' | '`' => true,
    c if c.is_whitespace() => true,
    _ => false,
  }
}

impl Reader {
  pub fn new(code: &str) -> Reader {
    let loc = Loc { line: 1, column: 1 };
    Reader {
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
        self.loc.column = 0;
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
      }
      self.unpin();
    }

    // String literals
    if self.current_char_def() == '\"' {
      self.move_next();
      while !self.at_eof() {
        let chr = self.get_then_move();
        if chr == '\"' {
          break;
        } else {
          builder.push(chr);
        }
      }

      return Tok::Str(builder.clone(), self.get_loc());
    }

    match self.current_char_def() {
      '(' => {
        self.move_next();
        return Tok::OpenParen(self.get_loc());
      }
      ')' => {
        self.move_next();
        return Tok::CloseParen(self.get_loc());
      }
      '[' => {
        self.move_next();
        return Tok::OpenBracket(self.get_loc());
      }
      ']' => {
        self.move_next();
        return Tok::CloseBracket(self.get_loc());
      }
      '{' => {
        self.move_next();
        return Tok::OpenBrace(self.get_loc());
      }
      '}' => {
        self.move_next();
        return Tok::CloseBrace(self.get_loc());
      }
      '\'' => {
        self.move_next();
        return Tok::Quote(self.get_loc());
      }
      _ => {}
    }

    if self.at_eof() {
      return Tok::Eof(self.get_loc());
    }

    while !self.at_eof() && !is_delim(self.current_char_def()) {
      builder.push(self.get_then_move());
    }

    if builder.len() > 0 {
      return Tok::Atom(builder.clone(), self.get_loc());
    }

    panic!("Unexpected character: '{:?}'", self.current_char_def());
  }

  pub fn next_expr(&mut self) -> Node {
    let mut tok = self.next_token();

    let mut quoted = false;
    match tok {
      Tok::Quote(_) => {
        quoted = true;
        tok = self.next_token();
      }
      _ => {}
    }

    return match tok {
      Tok::Eof(loc) => Node::Unit(NodeInfo::loc(loc)),

      Tok::Atom(a, loc) => Node::AtomLit(a, NodeInfo::loc(loc)),
      Tok::Number(n, loc) => Node::NumberLit(n, NodeInfo::loc(loc)),
      Tok::Str(s, loc) => Node::StringLit(s, NodeInfo::loc(loc)),
      Tok::Bool(b, loc) => Node::BoolLit(b, NodeInfo::loc(loc)),

      Tok::OpenParen(loc) => {
        let mut ns = Vec::<Node>::new();
        while !self.at_eof() {
          let sub_node = self.next_expr();
          match sub_node {
            Node::Unit(_) => return Node::List(ns, NodeInfo::loc(loc)),
            _ => ns.push(sub_node),
          };
        }

        if self.at_eof() {
          panic!("Unbalanced braces starting at line: {:?}", loc.line)
        }

        return Node::List(ns, NodeInfo::loc(loc));
      }

      Tok::CloseParen(loc) => return Node::Unit(NodeInfo::loc(loc)),

      _ => panic!("What? {:?}", tok),
    };
  }

  pub fn next_progn(&mut self) -> Node {
    let mut ns = Vec::<Node>::new();

    while !self.at_eof() {
      let expr = self.next_expr();
      match expr {
        Node::Unit(_) => {
          break;
        }
        _ => ns.push(expr),
      }
    }

    for n in &ns {
      println!("node: {}", n);
    }

    Node::Progn(ns, NodeInfo::new())
  }
}

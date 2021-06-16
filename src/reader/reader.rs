use super::super::reader::ast::*;

pub enum Tok {
  Eof,
  Number(f32),

}

pub fn read_token(iter: &mut std::str::Chars<'_>) {

}

pub fn read_progn<'a>(iter: &mut std::str::Chars<'a>) -> Node {
  let mut xs = Vec::<Node>::new();
  xs.push(Node::Atom("Test".to_string()));
  Node::Progn(xs)
}

pub fn read_code(code: &str) -> Node {
  let mut iter = code.chars();
  read_progn(&mut iter)
}
use super::super::reader::reader::{Tok};

pub enum Node {
  Atom(String),
  Progn(Vec<Node>),
  List(Vec<Node>),
}
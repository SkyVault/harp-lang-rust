use super::super::evaluator::value::Value;
use super::super::reader::reader::{Loc, Tok};
use std::fmt::*;

pub const QUOTED: u8 = 0b00000001;

#[derive(Debug, PartialEq)]
pub struct NodeInfo {
  pub flags: u8,
  pub loc: Loc,
}

impl Display for NodeInfo {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    write!(f, "{:#02X}", self.loc.line)
  }
}

impl NodeInfo {
  pub fn new() -> NodeInfo {
    NodeInfo {
      flags: 0u8,
      loc: Loc::blank(),
    }
  }

  pub fn loc(loc: Loc) -> NodeInfo {
    NodeInfo {
      flags: 0u8,
      loc: loc,
    }
  }

  pub fn new_flags(flags: u8) -> NodeInfo {
    NodeInfo {
      flags,
      loc: Loc::blank(),
    }
  }
}

#[derive(Debug, PartialEq)]
pub enum Node {
  Unit(NodeInfo),
  AtomLit(String, NodeInfo),
  StringLit(String, NodeInfo),
  NumberLit(f64, NodeInfo),
  BoolLit(bool, NodeInfo),
  Progn(Vec<Node>, NodeInfo),
  List(Vec<Node>, NodeInfo),
}

pub fn to_str(
  f: &mut std::fmt::Formatter<'_>,
  node: &Node,
  indent: String,
) -> std::result::Result<(), std::fmt::Error> {
  match node {
    Node::Unit(i) => write!(f, "{}    U", i),
    Node::AtomLit(s, i) => write!(f, "{}:{} A: {}", i, indent, s),
    Node::StringLit(s, i) => write!(f, "{}:{} S: {}", i, indent, s),
    Node::NumberLit(n, i) => write!(f, "{}:{} N: {}", i, indent, n),
    Node::BoolLit(b, i) => write!(f, "{}:{} B: {}", i, indent, b),
    Node::Progn(ns, i) => {
      write!(f, "{}:{} Progn:\n", i, indent);
      for n in ns {
        let mut next_indent = String::from("  ");
        next_indent.push_str(&indent.to_string());
        to_str(f, n, next_indent.to_string()).unwrap();
        write!(f, "\n");
      }
      write!(f, "")
    }
    Node::List(ns, i) => {
      write!(f, "{}:{} List:\n", i, indent);
      for n in ns {
        let mut next_indent = String::from("  ");
        next_indent.push_str(&indent.to_string());
        to_str(f, n, next_indent.to_string()).unwrap();
        write!(f, "\n");
      }
      write!(f, "")
    }
  }
}

pub fn to_value(node: &Node) -> Value {
  match node {
    Node::Unit(_) => Value::Unit,
    Node::AtomLit(s, _) => Value::Atom(s.clone()),
    Node::StringLit(s, _) => Value::String(s.clone()),
    Node::NumberLit(n, _) => Value::Number(n.clone()),
    Node::BoolLit(b, _) => Value::Bool(b.clone()),
    Node::List(xs, _) => Value::List(xs.iter().map(|n| to_value(n)).collect()),
    v => panic!("Not supported yet '{}'", v),
    // Progn(Vec<Node>, NodeInfo) =>
    // Node::List(xs, _) => {}
  }
}

impl Display for Node {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    to_str(f, self, "".to_string())
  }
}

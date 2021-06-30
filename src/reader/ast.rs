use super::super::reader::reader::{Loc, Tok};

pub const QUOTED: u8 = 0b00000001;

#[derive(Debug, PartialEq)]
pub struct NodeInfo {
  pub flags: u8,
  pub loc: Loc,
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

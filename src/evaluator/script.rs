use crate::evaluator::opcodes::Opcode;
use crate::evaluator::quick_eval::{qeval_expr, qeval_value};
use crate::evaluator::value::Value;
use std::collections::HashMap;

use std::io;
use std::io::*;

pub struct Script {
  pub constants: Vec<Value>,
  pub instructions: Vec<Opcode>,
}

impl Script {
  pub fn new() -> Script {
    Script {
      constants: Vec::new(),
      instructions: Vec::new(),
    }
  }

  pub fn clone(&self) -> Script {
    Script {
      constants: self.constants.clone(),
      instructions: self.instructions.clone(),
    }
  }

  pub fn contains_const(&mut self, v: &Value) -> Option<usize> {
    self.constants.iter().position(|b| b == v)
  }

  pub fn new_const(&mut self, v: &Value) -> usize {
    self.constants.push(v.clone());
    self.constants.len() - 1
  }

  pub fn new_inst(&mut self, op: Opcode) {
    self.instructions.push(op);
  }
}

use crate::evaluator::value::Value;

use std::fmt;
use std::fmt::*;

#[derive(Clone)]
pub enum Opcode {
    Push(Value),
    Pop,
    Const(usize),
    Call(usize),
    Jump(usize),
    // Label(usize), // Does this really need to be an opcode?
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Opcode::Push(value) => write!(f, "Push({})", value),
            Opcode::Pop => write!(f, "Pop"),
            Opcode::Const(index) => write!(f, "Const({})", index),
            Opcode::Call(args) => write!(f, "Call(#args: {})", args),
            Opcode::Jump(new_pc) => write!(f, "Jump(#addr: {})", new_pc),
        }
    }
}

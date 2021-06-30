use crate::evaluator::value::Value;

#[derive(Debug, Clone)]
pub enum Opcode {
    Push(Value),
    Pop,
    Const(usize),
    Call(usize),
}

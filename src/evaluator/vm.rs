use crate::evaluator::opcodes::Opcode;
use crate::evaluator::script::Script;
use crate::evaluator::value::Value;
use crate::get_value_from_env;

pub struct Vm {
  pub stack: Vec<Value>,
  pub pc: usize,
}

impl Vm {
  pub fn new() -> Vm {
    Vm {
      stack: Vec::new(),
      pc: 0,
    }
  }

  pub fn get_args(&mut self, num_args: usize) -> Vec<Value> {
    let mut results = Vec::new();
    while results.len() < num_args {
      match self.stack.pop() {
        Some(v) => results.push(v),
        None => panic!("Stack underflow!"),
      }
    }
    return results;
  }

  pub fn eval(&mut self, env: &mut Value, value: Value) -> Value {
    match value {
      Value::Number(_) | Value::String(_) | Value::Bool(_) | Value::Unit | Value::NativeFunc(_) => {
        value
      }
      Value::Atom(name) => match get_value_from_env(&name, env) {
        Some(value) => value,
        None => Value::Unit,
      },
      _ => panic!("Unhandled value"),
    }
  }

  pub fn eval_script(&mut self, env: &mut Value, script: Script) -> Value {
    while self.pc < script.instructions.len() {
      let opcode = &script.instructions[self.pc];

      match opcode {
        Opcode::Push(value) => {
          self.stack.push(value.clone());
          self.pc += 1;
        }

        Opcode::Const(index) => {
          self.stack.push(script.constants[*index].clone());
          self.pc += 1
        }

        Opcode::Call(num_args) => {
          let atom = self.stack.pop();
          match atom {
            Some(Value::Atom(lexeme)) => match get_value_from_env(&lexeme, env) {
              Some(Value::NativeFunc(callable)) => {
                let args = self.get_args(*num_args);
                let result = callable(self, args, env);
                self.stack.push(result);
              }
              None => panic!("Function {:?} does not exist in environment", lexeme),
              _ => panic!("'{:?}' is not a callable value", lexeme),
            },
            _ => panic!("Failed to call function"),
          }
          self.pc += 1;
        }
        _ => todo!(),
      }
    }

    match self.stack.pop() {
      Some(value) => value,
      None => Value::Unit,
    }
  }
}

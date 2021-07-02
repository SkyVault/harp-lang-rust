use crate::evaluator::opcodes::Opcode;
use crate::evaluator::vm::Vm;
use std::collections::HashMap;

use std::fmt;
use std::fmt::*;

#[derive(Clone)]
pub enum Value {
  Unit,
  Number(f64),
  String(String),
  Atom(String),
  Bool(bool),
  NativeFunc(fn(&mut Vm, Vec<Value>, &mut Value) -> Value),
  Env(Vec<HashMap<String, Value>>),
}

impl fmt::Display for Value {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Value::Unit => write!(f, "()"),
      Value::Number(n) => write!(f, "{}", n),
      Value::String(s) => write!(f, "{}", s),
      Value::Atom(a) => write!(f, "{}", a),
      Value::Bool(b) => write!(f, "{}", if *b { "#t" } else { "#f" }),
      // Value::NativeFunc(f) => write!(f, "{:?}", f),
      Value::Env(env) => write!(f, "Env"),
      _ => panic!("Unhandled value"),
    }
  }
}

impl PartialEq for Value {
  fn eq(&self, other: &Value) -> bool {
    match (self, other) {
      (Value::Number(a), Value::Number(b)) => a == b,
      (Value::String(a), Value::String(b)) => a == b,
      (Value::Atom(a), Value::Atom(b)) => a == b,
      (Value::Bool(a), Value::Bool(b)) => a == b,
      // (Value::Func(_), Value::Func(_)) => todo!(),
      _ => false,
    }
  }
}

pub fn get_value_from_env(name: &String, env: &mut Value) -> Option<Value> {
  match env {
    Value::Env(scope_list) => {
      for scope in scope_list.iter().rev() {
        if scope.contains_key(name) {
          return Some(scope[name].clone());
        }
      }
      return None;
    }
    _ => panic!("Expected environment got {}", env),
  }
}

pub fn put_value_into_env(name: &String, value: &Value, env: &mut Value) {
  match env {
    Value::Env(scopes) => {
      let len = scopes.len() - 1;
      scopes[len].insert(name.to_string(), value.clone());
    }
    _ => {
      panic!("Expected an environment");
    }
  }
  return ();
}

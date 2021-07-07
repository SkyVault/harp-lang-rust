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
  // value, next
  List(Vec<Value>),
  Do(Vec<Value>),
  NativeFunc(fn(Vec<Value>, &mut EnvHead) -> Value),
  Func(String, Vec<String>, Box<Value>),
  Env(Vec<HashMap<String, Value>>),
}

impl fmt::Debug for Value {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    return write!(f, "{}", self);
  }
}

impl fmt::Display for Value {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Value::Unit => write!(f, "()"),
      Value::Number(n) => write!(f, "{}", n),
      Value::String(s) => write!(f, "{}", s),
      Value::Atom(a) => write!(f, "{}", a),
      Value::Bool(b) => write!(f, "{}", if *b { "#t" } else { "#f" }),
      Value::Do(xs) => {
        write!(f, "Do(");
        for (i, node) in xs.iter().enumerate() {
          write!(f, "{}", node);
          if i < xs.len() - 1 {
            write!(f, " ");
          }
        }
        write!(f, ")")
      }
      Value::List(xs) => {
        write!(f, "List(");
        for (i, node) in xs.iter().enumerate() {
          write!(f, "{}", node);
          if i < xs.len() - 1 {
            write!(f, " ");
          }
        }
        write!(f, ")")
      }
      Value::Env(env) => write!(f, "Env"),
      Value::NativeFunc(_) => write!(f, "NativeFunc"),
      Value::Func(name, args, _progn) => {
        write!(f, "fn({} {:?})", name, args)
      }
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

pub struct EnvHead {
  values: HashMap<String, Value>,
  next: Option<Box<EnvHead>>,
}

impl EnvHead {
  pub fn new() -> EnvHead {
    EnvHead {
      values: HashMap::new(),
      next: None,
    }
  }

  pub fn set(&mut self, name: String, value: Value) {
    self.values.insert(name, value);
  }

  fn get_rec(&self, name: &String, env: &EnvHead) -> Option<Value> {
    if env.values.contains_key(name) {
      return Some(env.values[name].clone());
    } else {
      match &env.next {
        Some(next) => self.get_rec(name, &next),
        _ => None,
      }
    }
  }

  pub fn get(&self, name: String) -> Option<Value> {
    self.get_rec(&name, self)
  }

  pub fn push(self) -> EnvHead {
    EnvHead {
      values: HashMap::new(),
      next: Some(Box::new(self)),
    }
  }

  pub fn pop(self) -> Option<EnvHead> {
    match self.next {
      Some(lower) => Some(*lower),
      _ => None,
    }
  }
}

pub fn get_value_from_env(name: &String, env: &mut Value) -> Option<Value> {
  match env {
    Value::Env(scope_list) => {
      for scope in scope_list.iter().rev() {
        if scope.contains_key(name) {
          let v = scope[name].clone();
          return Some(v);
        }
      }
      return None;
    }
    _ => panic!("Expected environment got {}", env),
  }
}

pub fn put_value_into_env(name: &String, value: &Value, env: &mut Value) -> Value {
  match env {
    Value::Env(scopes) => {
      let len = scopes.len() - 1;
      scopes[len].insert(name.to_string(), value.clone());
      return value.clone();
    }
    _ => {
      panic!("Expected an environment");
    }
  }
}

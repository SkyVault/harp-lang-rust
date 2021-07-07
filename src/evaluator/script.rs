use crate::evaluator::opcodes::Opcode;
use crate::evaluator::quick_eval::{qeval_expr, qeval_value};
use crate::evaluator::value::put_value_into_env;
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

pub fn make_std_env() -> Value {
  let mut global_scope = HashMap::<String, Value>::new();

  let mut add_global = |ident: &str, v: Value| {
    global_scope.insert(ident.to_string(), v);
  };

  // add_global("*version*", Value::String(String::from("0.0.0")));

  // add_global(
  //   "read-string",
  //   Value::NativeFunc(|args, _env| {
  //     let prompt = if args.len() == 1 {
  //       match &args[0] {
  //         Value::String(lexeme) => lexeme.clone(),
  //         _ => "".to_string(),
  //       }
  //     } else {
  //       "".to_string()
  //     };

  //     let mut value = String::new();
  //     print!("{}", prompt);
  //     io::stdout().lock().flush().unwrap();
  //     io::stdin()
  //       .read_line(&mut value)
  //       .expect("Error: unable to read user input");

  //     return Value::String(value.trim().to_string());
  //   }),
  // );

  // add_global(
  //   "defun",
  //   Value::NativeFunc(|args, env| {
  //     let atom = &args[0];
  //     let params = &args[1];
  //     let progn = &args[2];

  //     match (atom, params) {
  //       (Value::Atom(name), Value::List(ps)) => {
  //         let mut params_names = Vec::<String>::new();

  //         for node in ps {
  //           match node {
  //             Value::Atom(name) => {
  //               params_names.push(name.clone());
  //             }
  //             v => {
  //               panic!("defun expects arguments to be atoms not: {}", v);
  //             }
  //           }
  //         }

  //         let fun = Value::Func(name.to_string(), params_names, Box::new(progn.clone()));
  //         return put_value_into_env(name, &fun, env);
  //       }
  //       _ => panic!("defun expected an atom and a list of parameters"),
  //     }
  //   }),
  // );

  // add_global(
  //   "set",
  //   Value::NativeFunc(|args, env| {
  //     if args.len() != 2 {
  //       panic!(
  //         "Wrong number of arguments, expected 2 but got {}",
  //         args.len()
  //       );
  //     }

  //     match &args[0] {
  //       Value::Atom(name) => {
  //         return put_value_into_env(name, &qeval_value(args[1].clone(), env), env)
  //       }
  //       _ => panic!("set expected an atom but got '{}'", &args[0]),
  //     }
  //   }),
  // );

  // add_global(
  //   "println",
  //   Value::NativeFunc(|args, env| {
  //     for value in args {
  //       print!("{} ", qeval_value(value, env));
  //     }
  //     print!("\n");
  //     return Value::Unit;
  //   }),
  // );

  // add_global(
  //   "print",
  //   Value::NativeFunc(|args, env| {
  //     for (i, value) in args.iter().enumerate() {
  //       print!("{}", qeval_value(value.clone(), env));
  //       if i < args.len() - 1 {
  //         print!(" ")
  //       }
  //     }
  //     return Value::Unit;
  //   }),
  // );

  // add_global(
  //   "+",
  //   Value::NativeFunc(|args, env| {
  //     let mut total = 0.0;
  //     for value in args {
  //       match qeval_value(value, env) {
  //         Value::Number(number) => total += number,
  //         v => panic!("'+' expected numbers... got '{}' instead", v),
  //       }
  //     }
  //     return Value::Number(total);
  //   }),
  // );

  // add_global(
  //   "*",
  //   Value::NativeFunc(|args, env| {
  //     let mut total = 1.0;
  //     for value in args {
  //       match qeval_value(value, env) {
  //         Value::Number(number) => total *= number,
  //         v => panic!("'*' expected numbers.. got '{}' instead", v),
  //       }
  //     }
  //     return Value::Number(total);
  //   }),
  // );

  // add_global(
  //   "-",
  //   Value::NativeFunc(|args, env| {
  //     let mut total = 0.0;
  //     let mut first = true;
  //     for value in args {
  //       match qeval_value(value, env) {
  //         Value::Number(number) => {
  //           if first {
  //             first = false;
  //             total = number;
  //           } else {
  //             total -= number
  //           }
  //         }
  //         v => panic!("'-' expected numbers.. got '{}' instead", v),
  //       }
  //     }

  //     return Value::Number(total);
  //   }),
  // );

  // add_global(
  //   "/",
  //   Value::NativeFunc(|args, env| {
  //     let mut total = 1.0;
  //     let mut first = true;
  //     for value in args {
  //       match qeval_value(value, env) {
  //         Value::Number(number) => {
  //           if first {
  //             first = false;
  //             total = number;
  //           } else {
  //             total /= number
  //           }
  //         }
  //         v => panic!("'-' expected numbers.. got '{}' instead", v),
  //       }
  //     }
  //     return Value::Number(total);
  //   }),
  // );

  // add_global(
  //   "eq",
  //   Value::NativeFunc(|args, env| {
  //     let a = qeval_value(args[0].clone(), env);
  //     let b = qeval_value(args[1].clone(), env);
  //     return Value::Bool(a == b);
  //   }),
  // );

  // add_global(
  //   "not",
  //   Value::NativeFunc(|args, env| {
  //     match qeval_value(args[0].clone(), env) {
  //       Value::Bool(b) => return Value::Bool(!b),
  //       // Do we want (not '()) to return #t? seems odd, but if unit evals to false then it might make sense
  //       Value::Unit => return Value::Bool(true),
  //       v => panic!("Not expects a boolean, not '{}'", v),
  //     }
  //   }),
  // );

  // add_global(
  //   "if",
  //   Value::NativeFunc(|args, env| match qeval_value(args[0].clone(), env) {
  //     Value::Bool(boolean) => {
  //       if boolean {
  //         return qeval_value(args[1].clone(), env);
  //       } else {
  //         if args.len() > 2 {
  //           return qeval_value(args[2].clone(), env);
  //         } else {
  //           return Value::Unit;
  //         }
  //       }
  //     }
  //     v => panic!(
  //       "if expected its expression to evaluate to boolean, but got {}",
  //       v
  //     ),
  //   }),
  // );

  return Value::Env(vec![global_scope]);
}

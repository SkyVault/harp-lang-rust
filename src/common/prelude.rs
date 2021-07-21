use std::io::{stdout, Write};

use crossterm::{
  cursor::{DisableBlinking, EnableBlinking, MoveTo, RestorePosition, SavePosition},
  execute, ExecutableCommand, Result,
};

use crate::evaluator::value::{EnvHead, Value};
use crate::qeval_expr;
use crate::qeval_value;
use std::rc::Rc;

fn std_print(args: Vec<Value>, env: &mut EnvHead) -> Value {
  for (i, arg) in args.iter().enumerate() {
    print!("{}", qeval_value(arg.clone(), env));
    if i < args.len() - 1 {
      print!("\n");
    }
  }
  Value::Unit
}

fn std_print_ln(args: Vec<Value>, env: &mut EnvHead) -> Value {
  std_print(args, env);
  print!("\n");
  Value::Unit
}

pub fn std_set_cursor_pos(args: Vec<Value>, env: &mut EnvHead) -> Value {
  match &args[..] {
    [x, y] => match (qeval_value(x.clone(), env), qeval_value(y.clone(), env)) {
      (Value::Number(xpos), Value::Number(ypos)) => {
        stdout().execute(MoveTo(xpos as u16, ypos as u16)).unwrap();
      }
      _ => panic!("Expected x and y to be numbers"),
    },
    _ => panic!("Expected x and y to be numbers"),
  }

  Value::Unit
}

pub fn std_add(args: Vec<Value>, env: &mut EnvHead) -> Value {
  let mut total = 0.0;
  for arg in args {
    match qeval_value(arg, env) {
      Value::Number(num) => total += num,
      _ => panic!("'+' can only be used with numbers"),
    }
  }
  Value::Number(total)
}

pub fn std_mul(args: Vec<Value>, env: &mut EnvHead) -> Value {
  let mut total = 1.0;
  for arg in args {
    match qeval_value(arg, env) {
      Value::Number(num) => total *= num,
      v => panic!("Mul (*) can only be used with numbers, but got {}", v),
    }
  }
  Value::Number(total)
}

pub fn std_sub(args: Vec<Value>, env: &mut EnvHead) -> Value {
  let mut total = 0.0;
  for (i, arg) in args.iter().enumerate() {
    match qeval_value(arg.clone(), env) {
      Value::Number(num) => {
        if i == 0 {
          total = num
        } else {
          total -= num
        }
      }
      _ => panic!("'-' can only be used with numbers"),
    }
  }
  Value::Number(total)
}

pub fn std_eq(args: Vec<Value>, env: &mut EnvHead) -> Value {
  return args
    .into_iter()
    .reduce(|a, b| Value::Bool(qeval_value(a, env) == qeval_value(b, env)))
    .unwrap()
    .clone();
}

pub fn std_not(args: Vec<Value>, env: &mut EnvHead) -> Value {
  match &qeval_value(args[0].clone(), env) {
    &Value::Bool(value) => Value::Bool(!value),
    _ => Value::Bool(false),
  }
}

pub fn std_if(args: Vec<Value>, env: &mut EnvHead) -> Value {
  match qeval_value(args[0].clone(), env) {
    Value::Bool(boolean) => {
      if boolean {
        qeval_value(args[1].clone(), env)
      } else {
        if args.len() > 2 {
          qeval_value(args[2].clone(), env)
        } else {
          Value::Unit
        }
      }
    }
    v => panic!(
      "If expected its expression to evaluate to boolean, but got {}",
      v
    ),
  }
}

pub fn std_set(args: Vec<Value>, env: &mut EnvHead) -> Value {
  match &args[0] {
    Value::Atom(name) => {
      let value = qeval_value(args[1].clone(), env);
      env.set(name.to_string(), value.clone());
      value
    }
    v => panic!("Set expected an identifier, but got: {}", v),
  }
}

pub fn std_define(args: Vec<Value>, env: &mut EnvHead) -> Value {
  match &args[0] {
    Value::Atom(name) => {
      let value = qeval_value(args[1].clone(), env);
      if let Some(_) = env.get(name.to_string()) {
        panic!("{} is already defined", name);
      } else {
        env.set(name.to_string(), value.clone());
        value
      }
    }
    v => panic!("Def expected an identifier, but got: {}", v),
  }
}

pub fn std_defun(args: Vec<Value>, env: &mut EnvHead) -> Value {
  if args.len() < 3 {
    panic!("Defun expected a list of parameters and a body")
  }

  match &args[0] {
    Value::Atom(name) => {
      let params = &args[1];
      let progn: Vec<Value> = args[2..args.len()].to_vec();

      match params {
        Value::List(ps) => {
          let mut params_names: Vec<String> = Vec::new();
          for value in ps {
            match value {
              Value::Atom(value) => {
                params_names.push(value.clone());
              }
              v => panic!("Defun expects a list of parameters, got {}", v),
            }
          }

          let res = Value::Func(name.to_string(), params_names, Box::new(Value::Do(progn)));
          env.set(name.to_string(), res.clone());
          res
        }
        otherwise => panic!(
          "Defun expected a list of parameters, but got: {}",
          otherwise
        ),
      }
    }
    v => panic!("Set expected an identifier, but got: {}", v),
  }
}

pub fn std_lambda(args: Vec<Value>, env: &mut EnvHead) -> Value {
  if args.len() < 2 {
    panic!("Defun expected a list of parameters and a body")
  }

  let params = &args[0];
  let progn: Vec<Value> = args[1..args.len()].to_vec();

  match params {
    Value::List(ps) => {
      let mut params_names: Vec<String> = Vec::new();
      for value in ps {
        match value {
          Value::Atom(value) => {
            params_names.push(value.clone());
          }
          v => panic!("Lambda expects a list of parameters, got {}", v),
        }
      }
      return Value::Func("anon".to_string(), params_names, Box::new(Value::Do(progn)));
    }
    otherwise => panic!(
      "Lambda expected a list of parameters, but got: {}",
      otherwise
    ),
  }
}

pub fn make_std_env() -> EnvHead {
  let mut env = EnvHead::new();
  env.set("*version*".to_string(), Value::String("0.0.0".to_string()));

  // IO
  env.set("print".to_string(), Value::NativeFunc(std_print_ln));
  env.set("println".to_string(), Value::NativeFunc(std_print_ln));
  env.set(
    "io/set-cursor-pos".to_string(),
    Value::NativeFunc(std_set_cursor_pos),
  );

  // Math
  env.set("+".to_string(), Value::NativeFunc(std_add));
  env.set("-".to_string(), Value::NativeFunc(std_sub));
  env.set("*".to_string(), Value::NativeFunc(std_mul));

  // Logic
  env.set("eq".to_string(), Value::NativeFunc(std_eq));
  env.set("not".to_string(), Value::NativeFunc(std_not));
  env.set("if".to_string(), Value::NativeFunc(std_if));

  // Environment
  env.set("def".to_string(), Value::NativeFunc(std_define));
  env.set("set!".to_string(), Value::NativeFunc(std_set));

  // Loops

  // Functional
  env.set("lambda".to_string(), Value::NativeFunc(std_lambda));
  env.set("λ".to_string(), Value::NativeFunc(std_lambda));
  env.set("defun".to_string(), Value::NativeFunc(std_defun));

  return env;
}

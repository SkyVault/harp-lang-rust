use crate::evaluator::value::{EnvHead, Value};
use crate::qeval_expr;
use crate::qeval_value;

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

pub fn make_std_env() -> EnvHead {
  let mut env = EnvHead::new();
  env.set("*version*".to_string(), Value::String("0.0.0".to_string()));

  // IO
  env.set("print".to_string(), Value::NativeFunc(std_print_ln));
  env.set("println".to_string(), Value::NativeFunc(std_print_ln));

  // Math
  env.set("+".to_string(), Value::NativeFunc(std_add));
  env.set("-".to_string(), Value::NativeFunc(std_sub));

  // Logic
  env.set("eq".to_string(), Value::NativeFunc(std_eq));
  env.set("not".to_string(), Value::NativeFunc(std_not));
  env.set("if".to_string(), Value::NativeFunc(std_if));

  // Loops

  // Functional

  return env;
}

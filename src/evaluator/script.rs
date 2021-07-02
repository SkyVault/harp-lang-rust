use crate::evaluator::opcodes::Opcode;
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

  add_global(
    "read-string",
    Value::NativeFunc(|vm, args, env| {
      let prompt = if args.len() == 1 {
        match &args[0] {
          Value::String(lexeme) => lexeme.clone(),
          _ => "".to_string(),
        }
      } else {
        "".to_string()
      };

      let mut value = String::new();
      print!("{}", prompt);
      io::stdout().lock().flush().unwrap();
      io::stdin()
        .read_line(&mut value)
        .expect("Error: unable to read user input");

      return Value::String(value.trim().to_string());
    }),
  );

  add_global(
    "set",
    Value::NativeFunc(|vm, args, env| {
      if args.len() != 2 {
        panic!(
          "Wrong number of arguments, expected 2 but got {}",
          args.len()
        );
      }

      let atom = &args[0];
      let value = &args[1];

      match atom {
        Value::Atom(name) => {
          put_value_into_env(name, &vm.eval(env, value.clone()), env);
        }
        _ => panic!("set expected an atom but got '{}'", atom),
      }

      return Value::Unit;
    }),
  );

  add_global(
    "print",
    Value::NativeFunc(|vm, args, env| {
      for value in args {
        print!("{} ", vm.eval(env, value));
      }
      print!("\n");
      return Value::Unit;
    }),
  );

  add_global(
    "+",
    Value::NativeFunc(|vm, args, env| {
      let mut total = 0.0;
      for value in args {
        match vm.eval(env, value) {
          Value::Number(number) => total += number,
          v => panic!("'+' expected numbers... got '{}' instead", v),
        }
      }
      return Value::Number(total);
    }),
  );

  add_global(
    "*",
    Value::NativeFunc(|vm, args, env| {
      let mut total = 1.0;
      for value in args {
        match vm.eval(env, value) {
          Value::Number(number) => total *= number,
          v => panic!("'*' expected numbers.. got '{}' instead", v),
        }
      }
      return Value::Number(total);
    }),
  );

  add_global(
    "-",
    Value::NativeFunc(|vm, args, env| {
      let mut total = 0.0;
      let mut first = true;
      for value in args {
        match vm.eval(env, value) {
          Value::Number(number) => {
            if first {
              first = false;
              total = number;
            } else {
              total -= number
            }
          }
          v => panic!("'-' expected numbers.. got '{}' instead", v),
        }
      }

      return Value::Number(total);
    }),
  );

  add_global(
    "/",
    Value::NativeFunc(|vm, args, env| {
      let mut total = 1.0;
      let mut first = true;
      for value in args {
        match vm.eval(env, value) {
          Value::Number(number) => {
            if first {
              first = false;
              total = number;
            } else {
              total /= number
            }
          }
          v => panic!("'-' expected numbers.. got '{}' instead", v),
        }
      }

      return Value::Number(total);
    }),
  );

  add_global(
    "eq",
    Value::NativeFunc(|vm, args, env| {
      let a = vm.eval(env, args[0].clone());
      let b = vm.eval(env, args[0].clone());
      println!("TEST: {} == {}", a, b);
      return Value::Bool(a == b);
    }),
  );

  add_global(
    "not",
    Value::NativeFunc(|vm, args, env| {
      let aa = &args[0];
      match vm.eval(env, aa.clone()) {
        Value::Bool(b) => return Value::Bool(!b),
        // Do we want (not '()) to return #t? seems odd, but if unit evals to false then it might make sense
        Value::Unit => return Value::Bool(true),
        v => panic!("Not expects a boolean, not '{}'", v),
      }
    }),
  );

  add_global(
    "if",
    Value::NativeFunc(|vm, args, env| {
      match vm.eval(env, args[0].clone()) {
        Value::Bool(boolean) => {
          if boolean {
            return vm.eval(env, args[1].clone());
          } else {
            if args.len() > 2 {
              return vm.eval(env, args[2].clone());
            } else {
              return Value::Unit;
            }
          }
        }
        v => panic!(
          "if expected its expression to evaluate to boolean, but got {}",
          v
        ),
      }

      // let alternative = &args[2];
      return Value::Unit;
    }),
  );

  return Value::Env(vec![global_scope]);
}

use crate::evaluator::opcodes::Opcode;
use crate::evaluator::value::put_value_into_env;
use crate::evaluator::value::Value;
use std::collections::HashMap;

#[derive(Debug)]
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
    "set",
    Value::NativeFunc(|args, env| {
      if args.len() < 2 {
        panic!("Set requires a atom and value, just got: {:?}", args);
      }

      let atom = &args[0];
      let value = &args[1];

      match atom {
        Value::Atom(name) => {
          // put_value_into_env(name, value, &mut env);
        }
      }

      return Value::Unit;
    }),
  );

  add_global(
    "print",
    Value::NativeFunc(|args, env| {
      for value in args {
        print!("{} ", value);
      }
      print!("\n");
      return Value::Unit;
    }),
  );

  add_global(
    "+",
    Value::NativeFunc(|args, env| {
      let mut total = 0.0;
      for value in args {
        match value {
          Value::Number(number) => total += number,
          _ => panic!("Plus only works with numbers so far.. got {:?}", value),
        }
      }
      return Value::Number(total);
    }),
  );

  add_global(
    "*",
    Value::NativeFunc(|args, env| {
      let mut total = 1.0;
      for value in args {
        match value {
          Value::Number(number) => total *= number,
          _ => panic!("Plus only works with numbers so far.. got {:?}", value),
        }
      }
      return Value::Number(total);
    }),
  );

  add_global(
    "-",
    Value::NativeFunc(|args, env| {
      let mut total = 0.0;
      let mut first = true;
      for value in args {
        match value {
          Value::Number(number) => {
            if first {
              first = false;
              total = number;
            } else {
              total -= number
            }
          }
          _ => panic!("Plus only works with numbers so far.. got {:?}", value),
        }
      }

      return Value::Number(total);
    }),
  );

  add_global(
    "/",
    Value::NativeFunc(|args, env| {
      let mut total = 1.0;
      let mut first = true;
      for value in args {
        match value {
          Value::Number(number) => {
            if first {
              first = false;
              total = number;
            } else {
              total /= number
            }
          }
          _ => panic!("Plus only works with numbers so far.. got {:?}", value),
        }
      }

      return Value::Number(total);
    }),
  );

  return Value::Env(vec![global_scope]);
}

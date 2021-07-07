use crate::evaluator::value::{EnvHead, Value};

pub fn make_std_env() -> EnvHead {
  let mut env = EnvHead::new();
  env.set("*version*".to_string(), Value::String("0.0.0".to_string()));
  return env;
}

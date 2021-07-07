pub mod opcodes;
pub mod quick_eval;
pub mod script;
pub mod value;
pub mod vm;

use value::*;

#[test]
fn value_env_test() {
  let mut env = EnvHead::new();
  env.set("global".to_string(), Value::Number(123.0));
  assert_eq!(env.get("global".to_string()), Some(Value::Number(123.0)));

  let mut env = env.push();
  env.set("local".to_string(), Value::Bool(true));

  assert_eq!(env.get("global".to_string()), Some(Value::Number(123.0)));
  assert_eq!(env.get("local".to_string()), Some(Value::Bool(true)));

  match env.pop() {
    Some(env) => {
      assert_eq!(env.get("local".to_string()), None);
    }
    None => {}
  }
}

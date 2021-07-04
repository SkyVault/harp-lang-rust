/*
	This module does evaluation on the ast itself, this is useful for getting a running system going, but in the long run we would need to
	retire this module in favor of the vm and translator
*/

use crate::evaluator::value::Value;
use crate::get_value_from_env;
use crate::reader::ast::{to_value, Node};

pub fn qeval_value(value: Value, env: &mut Value) -> Value {
	match value {
		Value::Number(_) | Value::String(_) | Value::Bool(_) | Value::Unit => value,
		Value::Atom(name) => match get_value_from_env(&name, env) {
			Some(value) => value,
			None => {
				panic!("Undefind Variable {}", name)
			}
		},
		_ => panic!("Cannot evaluate value {}", value),
	}
}

pub fn qeval_expr(expr: &Node, env: &mut Value) -> Value {
	match expr {
		Node::Unit(_) => Value::Unit,
		Node::NumberLit(num, _) => Value::Number(*num),
		Node::StringLit(string, _) => Value::String(string.to_string()),

		Node::AtomLit(name, i) => match get_value_from_env(name, env) {
			Some(value) => value,
			None => panic!("{}: Cannot find {} in the environment", i, name),
		},

		Node::List(xs, i) => {
			match xs.get(0) {
				Some(Node::AtomLit(name, i)) => match get_value_from_env(&name, env) {
					Some(Value::NativeFunc(callable)) => {
						let mut args = Vec::<Value>::new();

						let mut skip = true;
						for n in xs.iter() {
							if skip {
								skip = false;
								continue;
							}
							args.push(to_value(n));
						}

						callable(args, env)

						// for node in xs.clone() {
						// 	args.push(to_value(node));
						// }
					}
					Some(v) => panic!("{} Illegal functio call, {} is {}", i, name, v),
					None => {
						panic!("{} Cannot find {} in the environment", i, name)
					}
				},
				Some(v) => panic!("{} Illegal function call {}", i, v),
				None => Value::Unit,
			}
		}

		otherwise => panic!("Cannot evaluate expression: {:?}", otherwise),
	}
}

pub fn qeval_progn(progn: &Node, env: &mut Value) -> Value {
	return match progn {
		Node::Progn(ns, _) => ns.iter().map(|e| qeval_expr(e, env)).last().unwrap(),
		_ => qeval_expr(progn, env),
	};
}

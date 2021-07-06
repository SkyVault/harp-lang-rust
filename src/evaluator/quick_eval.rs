/*
	This module does evaluation on the ast itself, this is useful for getting a running system going, but in the long run we would need to
	retire this module in favor of the vm and translator
*/

use crate::evaluator::value::{put_value_into_env, Value};
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
		Value::List(xs) => {
			let first = &xs[0];
			match first {
				Value::Atom(name) => match get_value_from_env(&name, env) {
					Some(Value::NativeFunc(callable)) => {
						let mut args = Vec::<Value>::new();
						for n in xs.iter().skip(1) {
							args.push(n.clone());
						}
						callable(args, env)
					}
					Some(Value::Func(_name, params, progn)) => {
						for (value, name) in xs.iter().skip(1).zip(params) {
							put_value_into_env(&name, value, env);
						}
						qeval_value(*progn, env)
					}
					Some(v) => panic!("Illegal function call. {} is {}", name, v),
					None => panic!("Undefined function {}", name),
				},
				v => panic!("Cannot function call on function {}", v),
			}
		}
		_ => panic!("Cannot evaluate value {}", value),
	}
}

pub fn qeval_expr(expr: &Node, env: &mut Value) -> Value {
	return qeval_value(to_value(expr), env);
}

pub fn qeval_progn(progn: &Node, env: &mut Value) -> Value {
	return match progn {
		Node::Progn(ns, _) => ns.iter().map(|e| qeval_expr(e, env)).last().unwrap(),
		_ => qeval_expr(progn, env),
	};
}

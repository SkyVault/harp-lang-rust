/*
	This module does evaluation on the ast itself, this is useful for getting a running system going, but in the long run we would need to
	retire this module in favor of the vm and translator
*/

use crate::evaluator::value::{EnvHead, Value};
use crate::reader::ast::{to_value, Node};

pub fn qeval_value(value: Value, env: &mut EnvHead) -> Value {
	match value {
		Value::Number(_)
		| Value::String(_)
		| Value::Bool(_)
		| Value::NativeFunc(_)
		| Value::Func(_, _, _)
		| Value::Unit => value,
		Value::Atom(name) => match env.get(name.clone()) {
			Some(value) => value,
			None => {
				println!("Undefind Variable {}", name);
				return Value::Unit;
			}
		},
		Value::Do(xs) => {
			let mut res: Vec<Value> = Vec::new();

			for expr in xs {
				res.push(qeval_value(expr, env));
			}

			match res.last() {
				Some(value) => value.clone(),
				None => Value::Unit,
			}
		}
		Value::List(xs) => {
			let first = &xs[0];
			match qeval_value(first.clone(), env) {
				Value::Func(_, params, progn) => {
					let mut scope = env.clone().push();
					for (value, name) in xs.iter().skip(1).zip(params) {
						scope.set(name, qeval_value(value.clone(), env));
					}
					qeval_value(*progn, &mut scope)
				}
				Value::NativeFunc(callable) => {
					let mut args = Vec::<Value>::new();
					for n in xs.iter().skip(1) {
						args.push(n.clone());
					}
					callable(args, env)
				}
				Value::Atom(name) => match env.get(name.to_string()) {
					Some(Value::NativeFunc(callable)) => {
						let mut args = Vec::<Value>::new();
						for n in xs.iter().skip(1) {
							args.push(n.clone());
						}
						callable(args, env)
					}
					Some(Value::Func(_name, params, progn)) => {
						let mut scope = env.clone().push();
						for (value, name) in xs.iter().skip(1).zip(params) {
							scope.set(name, qeval_value(value.clone(), env));
						}
						qeval_value(*progn, &mut scope)
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

pub fn qeval_expr(expr: &Node, env: &mut EnvHead) -> Value {
	return qeval_value(to_value(expr), env);
}

pub fn qeval_progn(progn: &Node, env: &mut EnvHead) -> Value {
	return match progn {
		Node::Progn(ns, _) => ns.iter().map(|e| qeval_expr(e, env)).last().unwrap(),
		_ => qeval_expr(progn, env),
	};
}

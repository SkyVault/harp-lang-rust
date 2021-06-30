use crate::evaluator::opcodes::Opcode;
use crate::evaluator::script::Script;
use crate::evaluator::value::Value;
use crate::reader::ast::Node;

pub struct Translator {
  script: Script,
}

impl Translator {
  pub fn new() -> Translator {
    Translator {
      script: Script::new(),
    }
  }

  pub fn handle_const(&mut self, value: &Value) -> () {
    return match self.script.contains_const(&value) {
      Some(index) => self.script.new_inst(Opcode::Const(index)),
      _ => {
        let index = self.script.new_const(&value);
        self.script.new_inst(Opcode::Const(index))
      }
    };
  }

  pub fn translate_list(&mut self, list: &Vec<Node>) -> () {
    for node in list.iter().rev() {
      self.translate_expr(node);
    }

    self.script.new_inst(Opcode::Call(list.len() - 1))
  }

  pub fn translate_expr(&mut self, ns: &Node) -> () {
    match ns {
      Node::NumberLit(number, _) => self.handle_const(&Value::Number(number.clone())),
      Node::StringLit(lexeme, _) => self.handle_const(&Value::String(lexeme.clone())),
      Node::AtomLit(lexeme, _) => self.handle_const(&Value::Atom(lexeme.clone())),
      Node::List(xs, _) => self.translate_list(xs),

      Node::Unit(_) => {}
      _ => panic!("Can't translate expr node: {:?}", ns),
    }
  }

  pub fn progn_to_script(&mut self, node: Node) -> Script {
    match node {
      Node::Progn(xs, _) => {
        for sub in xs {
          self.translate_expr(&sub)
        }
      }
      otherwise => panic!("Progn to script expects a progn, but got: {:?}", otherwise),
    }
    self.script.clone()
  }
}

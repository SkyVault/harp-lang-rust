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

  pub fn transpile_if_expr(&mut self, expr: &Node, consequent: &Node, alternative: Option<&Node>) {
    // let start = self.script.instructions.len() - 1;

    self.translate_expr(expr);

    // self.translate_expr(&consequent);
  }

  pub fn translate_list(&mut self, list: &Vec<Node>) -> () {
    // Handle special forms, (until macros)
    match &list[..] {
      [Node::AtomLit(lexeme, _), expr, consequent] if lexeme == "if" => {
        self.transpile_if_expr(expr, consequent, None);
      }
      [Node::AtomLit(lexeme, _), expr, consequent, alternative] if lexeme == "if" => {
        self.transpile_if_expr(expr, consequent, Some(alternative));
      }
      _ => {}
    }

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
      Node::BoolLit(value, _) => self.script.new_inst(Opcode::Push(Value::Bool(*value))),
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

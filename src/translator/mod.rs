use evaluator::opcodes::Opcode;
use reader::ast::Node;

struct Translator {}

impl Translator {
    pub fn new() -> Translator {
        Translator {}
    }

    pub fn node_to_program(node: Node) -> Vec<Opcode> {}
}

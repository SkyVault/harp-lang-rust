use crate::evaluator::opcodes::Opcode;
use crate::evaluator::quick_eval::*;
use crate::evaluator::script::make_std_env;
use crate::evaluator::value::*;
use crate::reader::ast::Node;
use crate::reader::ast::NodeInfo;

mod evaluator;
mod reader;
mod translator;

fn main() {
    /*
    TODO:

    if expressions always evaluates both true and false branches
    this happens because both of those expressions get translated, so function calls get a call opcode
    we might want to wait until macros are a thing before fully fixing if,
    for now I can try handling it specially in the evaluator or probably translator
    */

    let test_code = "
        ;; (print (if #t 1 (print \"hello world\")))
        (print \"here: \" (+ 1 2 3))
    ";

    let mut reader = reader::reader::Reader::new(test_code);
    // let mut vm = evaluator::vm::Vm::new();

    let ast = reader.next_progn();

    let mut std_env = make_std_env();
    let result = qeval_progn(&ast, &mut std_env);
    println!("RESULT: {}", result);
}

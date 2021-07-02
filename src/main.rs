use crate::evaluator::opcodes::Opcode;
use crate::evaluator::script::make_std_env;
use crate::evaluator::value::get_value_from_env;

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
        (print (if #t 1 (print \"hello world\")))
    ";

    let mut trans = translator::translator::Translator::new();
    let mut reader = reader::reader::Reader::new(test_code);
    let mut vm = evaluator::vm::Vm::new();

    let ast = reader.next_progn();

    let script = trans.progn_to_script(ast);
    // println!("SCRIPT: {:?}", script);

    for (i, op) in script.instructions.iter().enumerate() {
        print!("({}):\t{}", i, op);
        match op {
            Opcode::Const(index) => println!("\t{}", script.constants[*index]),
            _ => println!(""),
        }
    }

    // let mut std_env = make_std_env();
    // let results = vm.eval_script(&mut std_env, script);

    // println!("RESULT: {}", results);
}

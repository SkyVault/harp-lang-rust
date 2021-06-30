use crate::evaluator::script::make_std_env;
use crate::evaluator::value::get_value_from_env;

mod evaluator;
mod reader;
mod translator;

fn main() {
    let test_code = "
        (set my-var (+ 1 2 3))
        (print my-var)
        (print \"banana\n(+ 1 2 =)\" (+ 1 2))
        (print \"hello world\" 69 (+ (* 3 3) 2 3))
        (+ (- 4 2) 6 2)
    ";

    let mut trans = translator::translator::Translator::new();
    let mut reader = reader::reader::Reader::new(test_code);
    let mut vm = evaluator::vm::Vm::new();

    let ast = reader.next_progn();

    let script = trans.progn_to_script(ast);
    println!("SCRIPT: {:?}", script);

    let mut std_env = make_std_env();
    let results = vm.eval(&mut std_env, script);

    println!("RESULT: {}", results);
}

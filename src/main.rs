use crate::evaluator::opcodes::Opcode;
use crate::evaluator::quick_eval::*;
use crate::evaluator::value::*;
use crate::reader::ast::Node;
use crate::reader::ast::NodeInfo;
use std::panic::catch_unwind;

use crate::common::prelude::make_std_env;

use rustyline::error::ReadlineError;
use rustyline::Editor;

mod common;
mod evaluator;
mod reader;
mod translator;

fn repl() {
    const HIST: &str = "harp-repl-history";

    let mut std_env = make_std_env();
    let mut rl = Editor::<()>::new();

    if rl.load_history(HIST).is_err() {}

    loop {
        match rl.readline("> ") {
            Ok(line) => {
                let ast = reader::reader::Reader::new(&line).next_progn();
                println!("{}", qeval_progn(&ast, &mut std_env));
            }

            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C (Exiting)");
                break;
            }

            Err(ReadlineError::Eof) => {
                println!("CTRL-D (Exiting)");
                break;
            }

            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history(HIST).unwrap();
}

fn main() {
    repl();
}

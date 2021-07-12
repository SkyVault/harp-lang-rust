use std::env;
use std::fs;

use crate::evaluator::quick_eval::*;
use crate::evaluator::value::*;

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

fn run_script(path: &String) {
    match fs::read_to_string(path) {
        Ok(s) => {
            let mut std_env = make_std_env();
            let progn = reader::reader::Reader::new(&s).next_progn();
            qeval_progn(&progn, &mut std_env);
        }
        Err(err) => panic!("{}", err),
    }
    // let mut std_env = make_std_env();
    // qeval_progn(progn: &Node, env: &mut EnvHead)
}

fn help() {
    println!("Harp Help")
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => repl(),
        2 => run_script(&args[1]),
        _ => help(),
    }
}

mod reader;

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Sub,
    Div,
    Mul,
}

#[derive(Debug, Clone, Copy)]
enum Value {
    Number(f64),
    Atom(usize),
    Func(Operator),
}

#[derive(Debug, Clone, Copy)]
enum Opcode {
    Push(Value),
    Pop,
    Call(usize),
}

#[derive(Debug)]
struct HarpVm {
    program: Vec<Opcode>,
    stack: Vec<Value>,

    pc: usize,
}

fn do_binop(op: Operator, a: Value, b: Value) -> Value {
    match (op, a, b) {
        (Operator::Add, Value::Number(left), Value::Number(right)) => Value::Number(left + right),
        (Operator::Sub, Value::Number(left), Value::Number(right)) => Value::Number(left - right),
        (Operator::Mul, Value::Number(left), Value::Number(right)) => Value::Number(left * right),
        (Operator::Div, Value::Number(left), Value::Number(right)) => Value::Number(left / right),
        _ => panic!("Bad operator application"),
    }
}

impl HarpVm {
    fn new() -> HarpVm {
        HarpVm {
            program: Vec::new(),
            stack: Vec::new(),
            pc: 0,
        }
    }

    fn load(self, program: Vec<Opcode>) -> HarpVm {
        HarpVm {
            program: program,
            pc: 0,
            ..self
        }
    }

    fn do_opcode(&mut self, opcode: Opcode) {
        match opcode {
            Opcode::Push(value) => {
                self.stack.push(value);
                self.pc += 1;
            }
            Opcode::Call(num_args) => {
                let mut args: Vec<Value> = Vec::new();
                for _ in 0..num_args {
                    match self.stack.pop() {
                        Some(value) => args.push(value),
                        None => panic!("Stack underflow! got {:?} stack {:?}", args, self.stack),
                    }
                }

                // TODO(Dustin): Do the function call

                self.pc += 1;
            }
            _ => panic!("Unhandled opcode: {:?}", opcode),
        }
    }

    fn eval(&mut self) {
        loop {
            if self.pc >= self.program.len() {
                return;
            }
            self.do_opcode(self.program[self.pc]);
        }
    }
}

fn main() {
    // let mut lexer = Lexer::new("3.14159 -32.1 .41 -.123");
    // println!("{:?}", lexer.next_token());
    // println!("{:?}", lexer.next_token());
    // println!("{:?}", lexer.next_token());
    // println!("{:?}", lexer.next_token());

    // let mut vm = HarpVm::new().load(vec![
    //     Opcode::Push(Value::Number(1.0)),
    //     Opcode::Push(Value::Number(2.0)),
    //     Opcode::Push(Value::Number(3.0)),
    //     Opcode::Push(Value::Atom(0)),
    //     Opcode::Call(3),
    // ]);

    // vm.eval();
    // println!("STACK: {:?}", vm.stack);
}

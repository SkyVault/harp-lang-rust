#[derive(Debug, Clone)]
pub enum Opcode {
    Push(Value),
    Pop,
    Call(usize),
}

#[derive(Debug, Clone)]
pub enum Value {
    Const(i64),
    Number(f64),
    Atom(usize),
    Func(Vec<Opcode>),
}

pub struct Script {
    constants: Vec<Value>,
    instructions: Vec<Opcode>,
}

pub struct Vm {
    pub stack: Vec<Value>,
    pub script: Option<Script>,
}

impl Vm {
    pub fn new() -> Vm {
        Vm {
            stack: Vec::new(),
            script: None,
        }
    }

    pub fn new_load(script: Script) -> Vm {
        let mut this = Vm::new();
        this.script = Some(script);
        return this;
    }
}

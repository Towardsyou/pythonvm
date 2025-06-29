use crate::vm::{
    code::{
        bytecode::{self, LOAD_NAME},
        code_object::CodeObject,
    },
    object::{HiObject, array_list::ArrayList, hi_list::HiList, hi_string::HiString},
};

pub struct Interpreter {
    _stack: ArrayList<HiObject>,
    _consts: HiList,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            _stack: ArrayList::new(256),
            _consts: HiList::new(256),
        }
    }

    pub fn run(&mut self, code: CodeObject) {
        let mut pc = 0;
        let code_length = code._bytecodes.length();

        let mut _stack = ArrayList::new(code._stack_size as usize);
        let _consts = code._consts;
        println!("{:?}", code._bytecodes);

        while pc < code_length {
            let op_code = code._bytecodes.value()[pc];
            pc += 1;
            let op_arg = code._bytecodes.value()[pc] & 0xff;
            pc += 1;

            match op_code {
                bytecode::LOAD_CONST => _stack.push(_consts.get(op_arg as usize)),
                bytecode::LOAD_NAME => _stack.push(HiObject::HiNone),
                bytecode::CALL_FUNCTION => {
                    let v = _stack.pop();
                    println!("print called: {:?}", v);
                }
                bytecode::POP_TOP => {
                    _stack.pop();
                }
                bytecode::RETURN_VALUE => {
                    _stack.pop();
                }
                _ => {
                    panic!("Unknown op_code: {}", op_code);
                }
            }
        }
    }
}

use chrono::Utc;

use crate::vm::{
    code::{
        bytecode::{self},
        code_object::CodeObject,
    },
    object::{
        HiObject, array_list::ArrayList, hi_list::HiList,
    },
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
                    let now = Utc::now();
                    println!("{} print called: {:?}", now, v);
                }
                bytecode::POP_TOP => {
                    _stack.pop();
                }
                bytecode::RETURN_VALUE => {
                    _stack.pop();
                }
                bytecode::COMPARE_OP => {
                    let w = _stack.pop();
                    let v = _stack.pop();
                    match op_arg {
                        bytecode::GREATER => {
                            _stack.push(v.greater(w));
                        }
                        bytecode::GREATER_EQUAL => {
                            _stack.push(v.ge(w));
                        }
                        bytecode::LESS => {
                            _stack.push(v.less(w));
                        }
                        bytecode::LESS_EQUAL => {
                            _stack.push(v.le(w));
                        }
                        bytecode::EQUAL => {
                            _stack.push(v.equal(w));
                        }
                        bytecode::NOT_EQUAL => {
                            _stack.push(v.not_equal(w));
                        }
                        _ => {
                            println!("Unrecognized comparison operator {}", op_arg)
                        }
                    };
                }
                bytecode::POP_JUMP_IF_FALSE => {
                    let v = _stack.pop();
                    if let HiObject::HiFalse = v {
                        pc += op_arg as usize;
                    }
                }
                bytecode::JUMP_FORWARD => {
                    pc += op_arg as usize;
                }
                _ => {
                    panic!("Unknown op_code: {}", op_code);
                }
            }
        }
    }
}

use crate::vm::object::{HiObject, hi_list::HiList, hi_string::HiString};

#[derive(Clone, Debug)]
pub struct CodeObject {
    pub _argcount: i32,
    pub _posonlyargcount: i32,
    pub _kwonlyargcount: i32,
    pub _nlocals: i32,
    pub _stack_size: i32,
    pub _flag: i32,

    pub _bytecodes: HiString,
    pub _names: HiList,
    pub _consts: HiList,
    pub _var_names: HiList,

    pub _free_vars: HiList,
    pub _cell_vars: HiList,

    pub _co_name: HiString,
    pub _file_name: HiString,

    pub _lineno: i32,
    pub _notable: HiString,
}

impl CodeObject {
    pub fn new(
        _argcount: i32,
        _posonlyargcount: i32,
        _kwonlyargcount: i32,
        _nlocals: i32,
        _stack_size: i32,
        _flag: i32,
        _bytecodes: HiString,
        _names: HiList,
        _consts: HiList,
        _var_names: HiList,
        _free_vars: HiList,
        _cell_vars: HiList,
        _co_name: HiString,
        _file_name: HiString,
        _lineno: i32,
        _notable: HiString,
    ) -> CodeObject {
        CodeObject {
            _argcount: _argcount,
            _posonlyargcount: _posonlyargcount,
            _kwonlyargcount: _kwonlyargcount,
            _nlocals: _nlocals,
            _stack_size: _stack_size,
            _flag: _flag,
            _bytecodes: _bytecodes,
            _names: _names,
            _consts: _consts,
            _var_names: _var_names,
            _free_vars: _free_vars,
            _cell_vars: _cell_vars,
            _co_name: _co_name,
            _file_name: _file_name,
            _lineno: _lineno,
            _notable: _notable,
        }
    }

    pub fn greater(&self, o: HiObject) -> HiObject {
        HiObject::HiTrue
    }
    pub fn less(&self, o: HiObject) -> HiObject {
        HiObject::HiTrue
    }
    pub fn le(&self, o: HiObject) -> HiObject {
        HiObject::HiTrue
    }
    pub fn ge(&self, o: HiObject) -> HiObject {
        HiObject::HiTrue
    }
    pub fn equal(&self, o: HiObject) -> HiObject {
        HiObject::HiTrue
    }
    pub fn not_equal(&self, o: HiObject) -> HiObject {
        HiObject::HiTrue
    }
}

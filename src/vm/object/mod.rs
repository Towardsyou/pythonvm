use crate::vm::code::code_object;

pub mod array_list;
pub mod hi_integer;
pub mod hi_list;
pub mod hi_string;

#[derive(Clone, Debug)]
pub enum HiObject {
    HiCode(code_object::CodeObject),
    HiString(hi_string::HiString),
    HiInteger(hi_integer::HiInteger),
    HiList(hi_list::HiList),
    HiNone,
}

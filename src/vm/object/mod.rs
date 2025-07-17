use crate::vm::code::code_object;

pub mod array_list;
pub mod hi_integer;
pub mod hi_list;
pub mod hi_map;
pub mod hi_string;

#[derive(Clone, Debug)]
pub enum HiObject {
    HiCode(code_object::CodeObject),
    HiString(hi_string::HiString),
    HiInteger(hi_integer::HiInteger),
    HiList(hi_list::HiList),
    HiNone,
    HiTrue,
    HiFalse,
}

impl HiObject {
    pub fn greater(&self, o: HiObject) -> HiObject {
        match self {
            HiObject::HiCode(code_object) => code_object.greater(o),
            HiObject::HiString(hi_string) => hi_string.greater(o),
            HiObject::HiInteger(hi_integer) => hi_integer.greater(o),
            HiObject::HiList(hi_list) => hi_list.greater(o),
            HiObject::HiNone => HiObject::HiFalse,
            HiObject::HiTrue => hi_integer::HiInteger::new(1).greater(o),
            HiObject::HiFalse => hi_integer::HiInteger::new(0).greater(o),
        }
    }
    pub fn less(&self, o: HiObject) -> HiObject {
        match self {
            HiObject::HiCode(code_object) => code_object.less(o),
            HiObject::HiString(hi_string) => hi_string.less(o),
            HiObject::HiInteger(hi_integer) => hi_integer.less(o),
            HiObject::HiList(hi_list) => hi_list.less(o),
            HiObject::HiNone => HiObject::HiFalse,
            HiObject::HiTrue => hi_integer::HiInteger::new(1).less(o),
            HiObject::HiFalse => hi_integer::HiInteger::new(0).less(o),
        }
    }
    pub fn equal(&self, o: HiObject) -> HiObject {
        match self {
            HiObject::HiCode(code_object) => code_object.equal(o),
            HiObject::HiString(hi_string) => hi_string.equal(o),
            HiObject::HiInteger(hi_integer) => hi_integer.equal(o),
            HiObject::HiList(hi_list) => hi_list.equal(o),
            HiObject::HiNone => {
                if let HiObject::HiNone = o {
                    return HiObject::HiTrue;
                }
                return HiObject::HiFalse;
            }
            HiObject::HiTrue => hi_integer::HiInteger::new(1).equal(o),
            HiObject::HiFalse => hi_integer::HiInteger::new(0).equal(o),
        }
    }
    pub fn not_equal(&self, o: HiObject) -> HiObject {
        match self {
            HiObject::HiCode(code_object) => code_object.greater(o),
            HiObject::HiString(hi_string) => hi_string.greater(o),
            HiObject::HiInteger(hi_integer) => hi_integer.greater(o),
            HiObject::HiList(hi_list) => hi_list.greater(o),
            HiObject::HiNone => {
                if let HiObject::HiNone = o {
                    return HiObject::HiFalse;
                }
                return HiObject::HiTrue;
            }
            HiObject::HiTrue => hi_integer::HiInteger::new(1).not_equal(o),
            HiObject::HiFalse => hi_integer::HiInteger::new(0).not_equal(o),
        }
    }
    pub fn ge(&self, o: HiObject) -> HiObject {
        match self {
            HiObject::HiCode(code_object) => code_object.greater(o),
            HiObject::HiString(hi_string) => hi_string.greater(o),
            HiObject::HiInteger(hi_integer) => hi_integer.greater(o),
            HiObject::HiList(hi_list) => hi_list.greater(o),
            HiObject::HiNone => HiObject::HiFalse,
            HiObject::HiTrue => hi_integer::HiInteger::new(1).ge(o),
            HiObject::HiFalse => hi_integer::HiInteger::new(0).ge(o),
        }
    }
    pub fn le(&self, o: HiObject) -> HiObject {
        match self {
            HiObject::HiCode(code_object) => code_object.le(o),
            HiObject::HiString(hi_string) => hi_string.le(o),
            HiObject::HiInteger(hi_integer) => hi_integer.le(o),
            HiObject::HiList(hi_list) => hi_list.le(o),
            HiObject::HiNone => HiObject::HiFalse,
            HiObject::HiTrue => hi_integer::HiInteger::new(1).le(o),
            HiObject::HiFalse => hi_integer::HiInteger::new(0).le(o),
        }
    }

    pub fn add(&self, o: HiObject) -> HiObject {
        match self {
            HiObject::HiString(hi_string) => todo!(),
            HiObject::HiInteger(hi_integer) => hi_integer.add(o),
            HiObject::HiList(hi_list) => todo!(),
            HiObject::HiNone => todo!(),
            HiObject::HiTrue => todo!(),
            HiObject::HiFalse => todo!(),
            HiObject::HiCode(code_object) => panic!("can't add code object"),
        }
    }
}

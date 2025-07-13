use crate::vm::object::HiObject;

#[derive(Clone, Debug, Copy)]
pub struct HiInteger {
    _value: i32,
}

impl HiInteger {
    pub fn new(value: i32) -> HiInteger {
        HiInteger { _value: value }
    }

    pub fn value(&self) -> i32 {
        self._value
    }

    pub fn print(&self) {
        println!("{}", self._value);
    }

    pub fn greater(&self, o: HiObject) -> HiObject {
        if let HiObject::HiInteger(hi) = o {
            if self._value > hi._value {
                return HiObject::HiTrue;
            }
            return HiObject::HiFalse;
        }
        panic!("Compare between HiInterger and {:?}", o.clone());
    }

    pub fn less(&self, o: HiObject) -> HiObject {
        if let HiObject::HiInteger(hi) = o {
            if self._value < hi._value {
                return HiObject::HiTrue;
            }
            return HiObject::HiFalse;
        }
        panic!("Compare between HiInterger and {:?}", o.clone());
    }

    pub fn le(&self, o: HiObject) -> HiObject {
        if let HiObject::HiInteger(hi) = o {
            if self._value <= hi._value {
                return HiObject::HiTrue;
            }
            return HiObject::HiFalse;
        }
        panic!("Compare between HiInterger and {:?}", o.clone());
    }

    pub fn ge(&self, o: HiObject) -> HiObject {
        if let HiObject::HiInteger(hi) = o {
            if self._value >= hi._value {
                return HiObject::HiTrue;
            }
            return HiObject::HiFalse;
        }
        panic!("Compare between HiInterger and {:?}", o.clone());
    }

    pub fn equal(&self, o: HiObject) -> HiObject {
        if let HiObject::HiInteger(hi) = o {
            if self._value == hi._value {
                return HiObject::HiTrue;
            }
            return HiObject::HiFalse;
        }
        panic!("Compare between HiInterger and {:?}", o.clone());
    }

    pub fn not_equal(&self, o: HiObject) -> HiObject {
        if let HiObject::HiInteger(hi) = o {
            if self._value != hi._value {
                return HiObject::HiTrue;
            }
            return HiObject::HiFalse;
        }
        panic!("Compare between HiInterger and {:?}", o.clone());
    }
}

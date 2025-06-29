#[derive(Clone, Debug)]
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
}

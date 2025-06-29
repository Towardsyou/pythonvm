use crate::vm::object::{HiObject, array_list::ArrayList};

#[derive(Clone, Debug)]
pub struct HiList {
    _value: ArrayList<HiObject>,
}

impl HiList {
    pub fn new(size: usize) -> HiList {
        HiList {
            _value: ArrayList::new(size),
        }
    }

    pub fn get(&self, index: usize) -> HiObject {
        self._value.get(index)
    }

    pub fn push(&mut self, o: HiObject) {
        self._value.push(o);
    }

    pub fn set(&mut self, index: usize, o: HiObject) {
        self._value.set(index, o);
    }
}

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

    pub fn print(&self) {
        println!("{:?}", self._value);
    }

    pub fn set(&mut self, index: usize, o: HiObject) {
        self._value.set(index, o);
    }

    pub fn greater(&self, o: HiObject) -> HiObject {
        todo!()
    }
    pub fn less(&self, o: HiObject) -> HiObject {
        todo!()
    }
    pub fn le(&self, o: HiObject) -> HiObject {
        todo!()
    }
    pub fn ge(&self, o: HiObject) -> HiObject {
        todo!()
    }
    pub fn equal(&self, o: HiObject) -> HiObject {
        todo!()
    }
    pub fn not_equal(&self, o: HiObject) -> HiObject {
        todo!()
    }
}

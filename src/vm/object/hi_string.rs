#[derive(Clone, Debug)]
pub struct HiString {
    _value: Vec<u8>,
}

impl HiString {
    pub fn new(value: Vec<u8>) -> HiString {
        HiString { _value: value }
    }

    pub fn value(&self) -> Vec<u8> {
        self._value.clone()
    }

    pub fn length(&self) -> usize {
        return self._value.len();
    }

    pub fn print(&self) {
        print!("{}", String::from_utf8_lossy(&self._value));
    }
}

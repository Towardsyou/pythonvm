#[derive(Clone, Debug)]
pub struct ArrayList<T> {
    _array: Vec<T>,
}

impl<T> ArrayList<T>
where
    T: Clone,
{
    pub fn new(size: usize) -> ArrayList<T> {
        ArrayList {
            _array: Vec::with_capacity(size),
        }
    }

    pub fn push(&mut self, t: T) {
        self._array.push(t);
    }

    pub fn pop(&mut self) -> T {
        match self._array.pop() {
            Some(v) => v,
            None => panic!("ArrayList is empty"),
        }
    }

    pub fn cap(&self) -> usize {
        self._array.capacity()
    }

    pub fn len(&self) -> usize {
        self._array.len()
    }

    pub fn get(&self, index: usize) -> T {
        if index >= self.len() {
            panic!(
                "Index out of range, current length: {}, want idx {}",
                self.len(),
                index
            );
        }
        self._array[index].clone()
    }

    pub fn set(&mut self, index: usize, value: T) {
        if index >= self.len() {
            panic!("Index out of range");
        }
        self._array[index] = value;
    }
}

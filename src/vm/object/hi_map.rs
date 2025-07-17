use crate::vm::object::HiObject;

pub struct MapEntry {
    pub key: HiObject,
    pub val: HiObject,
    del: bool,
}

pub struct Map {
    _entries: Vec<MapEntry>,
}

impl Map {
    pub fn new() -> Map {
        Map { _entries: vec![] }
    }

    pub fn put(&mut self, k: HiObject, v: HiObject) {
        for entry in self._entries.iter_mut() {
            if let HiObject::HiTrue = entry.key.equal(k.clone()) {
                entry.del = false;
                entry.val = v;
                return;
            }
        }
        self._entries.push(MapEntry {
            key: k,
            val: v,
            del: false,
        });
    }

    pub fn get(&self, k: HiObject) -> HiObject {
        for entry in self._entries.iter() {
            if entry.del {
                continue;
            }
            if let HiObject::HiTrue = entry.key.equal(k.clone()) {
                return entry.val.clone();
            }
        }
        return HiObject::HiNone;
    }

    pub fn remove(&mut self, k: HiObject) {
        for entry in self._entries.iter_mut() {
            if entry.del {
                continue;
            }
            if let HiObject::HiTrue = entry.key.equal(k.clone()) {
                entry.del = true;
                return;
            }
        }
    }
}

use core::panic;
use std::{fs::File, io::Read};

use chrono::DateTime;

use crate::vm::{
    code::code_object::CodeObject,
    object::{
        HiObject, array_list::ArrayList, hi_integer::HiInteger, hi_list::HiList,
        hi_string::HiString,
    },
};

const BUFFER_LEN: usize = 256;

pub struct BufferedInputStream {
    fp: File,
    sz_buffer: [u8; BUFFER_LEN],
    index: usize,
}

impl BufferedInputStream {
    pub fn new(filepath: &str) -> BufferedInputStream {
        let mut file = match File::open(filepath) {
            Err(e) => panic!("couldn't open {}: {}", filepath, e),
            Ok(file) => file,
        };
        let mut buf = [0u8; 256];
        match file.read(&mut buf[..]) {
            Ok(n) => println!("read {} bytes", n),
            Err(e) => panic!("error reading file {}", e),
        };
        BufferedInputStream {
            fp: file,
            sz_buffer: buf,
            index: 0,
        }
    }
    pub fn read(&mut self) -> u8 {
        if self.index < BUFFER_LEN {
            let v = self.sz_buffer[self.index];
            self.index += 1;
            return v;
        } else {
            self.index = 0;
            self.sz_buffer = match self.fp.read(&mut self.sz_buffer[..]) {
                Ok(_) => self.sz_buffer,
                Err(_) => panic!("Error reading file"),
            };
            let v = self.sz_buffer[self.index];
            self.index += 1;
            return v;
        };
    }

    pub fn read_int(&mut self) -> i32 {
        let b1: i32 = (self.read() & 0xff) as i32;
        let b2: i32 = (self.read() & 0xff) as i32;
        let b3: i32 = (self.read() & 0xff) as i32;
        let b4: i32 = (self.read() & 0xff) as i32;

        return b4 << 24 | b3 << 16 | b2 << 8 | b1;
    }

    pub fn read_double(&mut self) -> f64 {
        let mut buffer = [0u8; 8];

        self.fp
            .read_exact(&mut buffer)
            .expect("fail to read 8 bytes");
        let double_value = f64::from_le_bytes(buffer);

        return double_value;
    }

    pub fn unread(&mut self) {
        self.index -= 1;
    }
}

pub enum DebugLevel {
    INFO,
}

pub struct BinaryFileParser {
    file_stream: BufferedInputStream,
    cur: usize,

    _string_table: ArrayList<HiString>,
    _cache: ArrayList<HiObject>,

    _debug_level: DebugLevel,
}

impl BinaryFileParser {
    pub fn new(file_stream: BufferedInputStream) -> BinaryFileParser {
        BinaryFileParser {
            file_stream: file_stream,
            cur: 0,
            _string_table: ArrayList::new(32),
            _cache: ArrayList::new(32),
            _debug_level: DebugLevel::INFO,
        }
    }

    pub fn parse(&mut self) -> CodeObject {
        let magic_number = self.file_stream.read_int();
        println!("magic number is 0x{:x}", magic_number);
        let file_flag = self.file_stream.read_int();
        println!("file flag is 0x{:x}", file_flag);
        let moddate: i32 = self.file_stream.read_int();
        let naive_datetime =
            DateTime::from_timestamp(moddate as i64, 0).expect("Invalid NaiveDateTime timestamp");
        println!("moddate is {}", naive_datetime);
        let file_size = self.file_stream.read_int();
        println!("size of source file is {}", file_size);

        let mut object_type = self.file_stream.read();
        let ref_flag = (object_type & 0x80) != 0;
        object_type &= 0x7f;

        if object_type == 'c' as u8 {
            let mut index = 0;
            if ref_flag {
                index = self._cache.len();
                self._cache.push(HiObject::HiNone);
            }

            let result: CodeObject = self.get_code_object();

            if ref_flag {
                self._cache.set(index, HiObject::HiCode(result.clone()))
            }

            println!("Parse OK!");
            return result;
        }

        panic!("there should be a main code object")
    }
    pub fn get_string(&mut self, is_long: bool) -> HiString {
        let length: usize = match is_long {
            true => self.file_stream.read_int() as usize,
            false => self.file_stream.read() as usize,
        };
        let mut str_value: Vec<u8> = Vec::with_capacity(length);

        for _ in 0..length {
            str_value.push(self.file_stream.read());
        }

        return HiString::new(str_value);
    }

    pub fn get_byte_codes(&mut self) -> HiString {
        let mut object_type = self.file_stream.read();
        let ref_flag: bool = (object_type & 0x80) != 0;
        object_type &= 0x7f;
        assert_eq!(object_type as char, 's');

        let s = self.get_string(true);

        if ref_flag {
            self._cache.push(HiObject::HiString(s.clone()));
        }

        s
    }

    pub fn get_code_object(&mut self) -> CodeObject {
        let argcount = self.file_stream.read_int();
        let posonly_argcount = self.file_stream.read_int();
        let kwonly_argcount = self.file_stream.read_int();
        let nlocals = self.file_stream.read_int();
        let stack_size = self.file_stream.read_int();
        let flag = self.file_stream.read_int();

        let byte_codes = self.get_byte_codes();
        let consts = self.get_consts();
        let names = self.get_var_names();
        let var_names = self.get_var_names();
        let free_vars = self.get_free_vars();
        let cell_vars = self.get_cell_vars();

        let file_name = self.get_file_name();
        let module_name = self.get_name();
        let begin_line_no = self.file_stream.read_int();
        let lnotab = self.get_no_table();
        dbg!(format!("const: {:?}", &consts));

        return CodeObject::new(
            argcount,
            posonly_argcount,
            kwonly_argcount,
            nlocals,
            stack_size,
            flag,
            byte_codes,
            names,
            consts,
            var_names,
            free_vars,
            cell_vars,
            module_name,
            file_name,
            begin_line_no,
            lnotab,
        );
    }

    pub fn get_no_table(&mut self) -> HiString {
        let object_type = self.file_stream.read();

        if object_type != 's' as u8 && object_type != 't' as u8 {
            // panic!("Invalid object type for no table: {}", object_type)
            println!(
                "expect a string for no table, but got type: {}",
                object_type
            );
            self.file_stream.unread();
            return HiString::new(vec![]);
        }

        self.get_string(true)
    }

    pub fn get_name(&mut self) -> HiString {
        let mut object_type = self.file_stream.read();
        let ref_flag = (object_type & 0x80) != 0;
        object_type &= 0x7f;

        let s = match object_type as char {
            's' => self.get_string(true),
            't' => self.get_string(false),
            'z' => self.get_string(false),
            'Z' => self.get_string(false),
            'R' => self._string_table.get(self.file_stream.read_int() as usize),
            _ => panic!("Invalid string type {}", object_type),
        };

        if ref_flag {
            self._cache.push(HiObject::HiString(s.clone()))
        }

        return s;
    }

    pub fn get_consts(&mut self) -> HiList {
        self.try_to_get_tuple()
    }

    pub fn get_names(&mut self) -> HiList {
        self.try_to_get_tuple()
    }

    pub fn get_file_name(&mut self) -> HiString {
        self.get_name()
    }

    pub fn get_var_names(&mut self) -> HiList {
        self.try_to_get_tuple()
    }

    pub fn get_free_vars(&mut self) -> HiList {
        self.try_to_get_tuple()
    }

    pub fn get_cell_vars(&mut self) -> HiList {
        self.try_to_get_tuple()
    }

    pub fn get_tuple(&mut self) -> HiList {
        let length = self.file_stream.read();
        let mut l = HiList::new(length as usize);
        for _ in 0..length as usize {
            let mut object_type = self.file_stream.read();
            let ref_flag = (object_type & 0x80) != 0;
            object_type &= 0x7f;

            let obj = match object_type as char {
                'c' => {
                    let mut index = 0;
                    if ref_flag {
                        index = self._cache.len();
                        self._cache.push(HiObject::HiNone);
                    }
                    let o = HiObject::HiCode(self.get_code_object());
                    if ref_flag {
                        self._cache.set(index, o.clone());
                    }
                    o
                }
                'i' => HiObject::HiInteger(HiInteger::new(self.file_stream.read_int())),
                'N' => HiObject::HiNone,
                't' => HiObject::HiString(self.get_string(true)),
                'Z' => HiObject::HiString(self.get_string(false)),
                's' => HiObject::HiString(self.get_string(true)),
                'R' => {
                    HiObject::HiString(self._string_table.get(self.file_stream.read_int() as usize))
                }
                'T' => HiObject::HiTrue,
                'F' => HiObject::HiFalse,
                _ => {
                    panic!("Unknown object type: {}", object_type);
                }
            };

            if ref_flag && object_type != 'c' as u8 {
                self._cache.push(obj.clone());
            }
            l.push(obj)
        }

        l
    }

    pub fn try_to_get_tuple(&mut self) -> HiList {
        let mut object_type = self.file_stream.read();
        let ref_flag = (object_type & 0x80) != 0;
        object_type &= 0x7f;

        let result = match object_type as char {
            ')' => {
                let t = self.get_tuple();
                if ref_flag {
                    self._cache.push(HiObject::HiList(t.clone()));
                }
                t
            }
            'r' => {
                let index = self.file_stream.read_int();
                if let HiObject::HiList(t) = self._cache.get(index as usize) {
                    t
                } else {
                    panic!(
                        "expect HiList in cache, get: {:?}",
                        self._cache.get(index as usize)
                    )
                }
            }
            _ => {
                panic!("Unrecognized object type: {}", object_type);
            }
        };

        result
    }
}

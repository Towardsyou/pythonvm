use crate::vm::{
    code::binary_file_parser::{BinaryFileParser, BufferedInputStream},
    runtime::interpreter::Interpreter,
};

mod vm;

fn main() {
    println!("Hello, world!");
    // read one file path from the terminal param
    let mut args = std::env::args();
    let file_path = if args.len() != 2 {
        println!("Usage: hi <file_path>");
        "__pycache__/t.cpython-38.pyc".to_string()
    } else {
        args.nth(1).expect("Please supply a file path")
    };

    let stream = BufferedInputStream::new(&file_path);
    let mut parser = BinaryFileParser::new(stream);
    let code = parser.parse();

    let mut interpreter = Interpreter::new();
    interpreter.run(code);
}

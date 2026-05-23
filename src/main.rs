mod interpreter;
mod parser;

use interpreter::*;
use parser::*;

use std::fs::read_to_string;

fn main() {
    /*
    dbg!(parse_statements(
        &mut dbg!(tokenize(
            "String Str String.new()\n\
        Void Execute Method Strings Arguments\n\
        \\t String Str String.new()\
            "
        ))
        .iter()
        .peekable(),
        0,
    ));
    */

    dbg!(parse(&tokenize(
        // &read_to_string("./examples/Example3.lang").unwrap()
        &read_to_string("./examples/Main.lang").unwrap()
    )));
}

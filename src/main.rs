mod parser;
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

    dbg!(parse_statements(
        &mut dbg!(tokenize(&read_to_string("./examples/Main.lang").unwrap()))
            .iter()
            .peekable(),
        0,
    ));
}

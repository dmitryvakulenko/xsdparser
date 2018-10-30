pub mod cmd_line;
pub mod parser;

extern crate xml;

#[macro_use]
extern crate log;
extern crate simple_logger;

fn main() {
    let _ = simple_logger::init();

    let schemas = cmd_line::parse_cmd_line();
    let _ = parser::build_tree(&schemas[0]);
}



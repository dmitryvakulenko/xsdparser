pub mod cmd_line;
pub mod parser;

fn main() {
    let schemas = cmd_line::parse_cmd_line();
    parser::build_tree(&schemas[0])
}



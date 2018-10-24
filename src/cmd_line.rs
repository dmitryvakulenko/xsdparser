use std::string::String;
use std::vec::Vec;
use std::env::args;

pub fn parse_cmd_line() -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    let mut args = args();
    let mut first = true;
    while let Some(cur_arg) = args.next() {
        if first {
            first = false;
            continue;
        } else {
            res.push(cur_arg)
        }
    }

    return res
}


extern crate xml;

mod model;

use std::fs::File;
use std::io::BufReader;
use std::string::String;
use parser::xml::reader::{EventReader, XmlEvent};

pub fn build_tree(file_name: &String) {
    let file = File::open(file_name).unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    let mut nodes = Vec::new();
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                let new_node = model::Node::new();
                nodes.push(&new_node);
                if nodes.len() > 0 {
                    nodes.last_mut().unwrap().add_child(new_node)
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                nodes.pop();
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
}
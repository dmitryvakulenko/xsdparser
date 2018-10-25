extern crate xml;

mod model;

use std::fs::File;
use std::io::BufReader;
use std::string::String;
use parser::xml::reader::{EventReader, XmlEvent};

pub fn build_tree(file_name: &String) -> model::Node {
    let file = File::open(file_name).unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    let mut root = model::Node::new();
    let mut nodes = Vec::<&mut model::Node>::new();
    for e in parser {
        match e {
            c @ Ok(XmlEvent::StartElement { .. }) => {
                let mut new_node = model::Node::new();
                let add_node: &mut model::Node;
                {
                    add_node = match nodes.last_mut() {
                        Some(ref mut n) =>
                            n.add_child(new_node),
                        None =>
                            root.add_child(new_node)
                    };
                }
                nodes.push(add_node)
            },
            Ok(XmlEvent::EndElement { .. }) => {
                nodes.pop();
            },
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }

    return root;
}
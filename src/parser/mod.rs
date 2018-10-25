extern crate xml;

mod model;

use std::fs::File;
use std::io::BufReader;
use std::string::String;
use self::xml::reader::{EventReader, XmlEvent};
use std::rc::Rc;

pub fn build_tree(file_name: &String) -> model::Node {
    let file = File::open(file_name).unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    let mut root = model::Node::new();
    let mut nodes = Vec::<Rc<&mut model::Node>>::new();
    for e in parser {
        match e {
            c @ Ok(XmlEvent::StartElement { .. }) => {
                let mut new_node = model::Node::new();
                let added_node = match nodes.last_mut() {
                    Some(n) =>
                        n.add_child(new_node),
                    None =>
                        root.add_child(new_node)
                };
                nodes.push(Rc::new(added_node))
            }
            Ok(XmlEvent::EndElement { .. }) => {
                nodes.pop();
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }

    return root;
}
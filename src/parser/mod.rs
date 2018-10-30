mod model;

use std::fs::File;
use std::io::BufReader;
use std::string::String;
use xml::reader::{EventReader, XmlEvent};
use self::model::Node;

pub fn build_tree(file_name: &String) {
    debug!("Reading file {}", file_name);

    let file = File::open(file_name).unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    let mut root = Node::new();
    let mut parents = Vec::new();
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement {name, .. }) => {
                debug!("Element: {}", name);
                let new_node = Node::new();
                parents.push(new_node)
            }
            Ok(XmlEvent::EndElement { .. }) =>
                match parents.pop() {
                    Some(n) =>
                        match parents.last_mut() {
                            Some(last) => last.add_child(n),
                            None => root.add_child(n)
                        },
                    None => {}
                }
            Err(e) => {
                error!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
}
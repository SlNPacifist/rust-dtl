use super::Node;
use super::NodeType;
use std::path::Path;
use Context;

pub struct ExtendsNode {
    name: String,
    dynamic: bool
}

impl ExtendsNode {
    pub fn new(name: String) -> ExtendsNode {
        let s = name.trim();
        let mut count = 0;
        for ch in s.chars() {
            if ch == '"' {
                count += 1;
            }
        }
        if count == 0 {
            ExtendsNode { name: s.to_string(), dynamic: true }
        } else {
            if count != 2 {
                panic!("Oops! Need correct name"); // FIXME: asd
            } else {
                ExtendsNode { name: s.trim_matches('"').to_string(), dynamic: false }
            }
        }
    }

    pub fn name(&self) -> &Path {
        if !self.dynamic {
            Path::new(&self.name)
        } else {
            unimplemented!();
        }
    }
}

impl Node for ExtendsNode {
    fn node_type(&self) -> NodeType {
        NodeType::Extends
    }
    fn print(&self, level: u32) {
        for _ in 0..level {
            print!("  ");
        }
        println!("extends: {:?}", self.name);
    }
    fn render(&self, _context: &Context) -> String {
        "".to_string()
    }
}
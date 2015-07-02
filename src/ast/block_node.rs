use std::rc::Rc;

use super::Node;
use super::NodeType;
use Context;

pub struct BlockNode {
    name: String,
    content: Vec<Rc<Box<Node>>>,
}

impl BlockNode {
    pub fn new(name: String, nodes: Vec<Rc<Box<Node>>>) -> Self {
        BlockNode { name: name, content: nodes }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Node for BlockNode {
    fn node_type(&self) -> NodeType {
        NodeType::Block
    }
    fn print(&self, level: u32) {
        for _ in 0..level {
            print!("  ");
        }
        println!("block: {:?}", self.name);
        for blk in self.content.iter() {
            blk.print(level + 1);
        }
    }
    fn render(&self, context: &Context) -> String {
        let mut res = String::new();
        for node in self.content.iter() {
            res.push_str(&node.render(context));
        }
        res
    }
}
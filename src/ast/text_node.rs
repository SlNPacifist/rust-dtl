use super::Node;
use super::NodeType;
use super::super::Context;

pub struct TextNode {
    value: String,
}

impl TextNode {
    pub fn new(value: &str) -> TextNode {
        TextNode { value: value.to_string() }
    }
}

impl Node for TextNode {
    fn node_type(&self) -> NodeType {
        NodeType::Text
    }
    fn print(&self, level: u32) {
        for _ in 0..level {
            print!("  ");
        }
        println!("text: {:?}", self.value);
    }
    fn render(&self, _context: &Context) -> String {
        self.value.to_string()
    }
}
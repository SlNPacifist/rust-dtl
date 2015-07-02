use scanner::Token;
use filters::FilterExpression;
use super::Node;
use super::NodeType;
use Context;

pub struct VariableNode {
    expr: FilterExpression,
}

impl VariableNode {
    pub fn new(token: &Token) -> VariableNode {
        VariableNode { expr: FilterExpression::new(token) }
    }
}

impl Node for VariableNode {
    fn node_type(&self) -> NodeType {
        NodeType::Variable
    }
    fn print(&self, level: u32) {
        for _ in 0..level {
            print!("  ");
        }
        println!("var: {:?}", self.expr.var());
    }
    fn render(&self, context: &Context) -> String {
        self.expr.render(context)
    }
}
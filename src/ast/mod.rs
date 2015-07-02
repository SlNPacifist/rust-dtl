use std::vec::Vec;
use std::boxed::Box;
use std::io::Result;
use std::io::{Error, ErrorKind};
use std::slice::Iter;
use std::rc::Rc;
use mopa::Any;

use scanner::Token;
use scanner::TokenId;
use super::Context;

mod block_node;
mod extends_node;
mod text_node;
mod variable_node;

pub use self::block_node::BlockNode;
pub use self::extends_node::ExtendsNode;
pub use self::text_node::TextNode;
pub use self::variable_node::VariableNode;

#[derive(PartialEq, PartialOrd)]
pub enum NodeType {
    Block,
    Extends,
    Text,
    Variable,
}

pub trait Node: Any {
    fn node_type(&self) -> NodeType;
    fn print(&self, level: u32);
    fn render(&self, &Context) -> String;
}

mopafy!(Node);

pub fn build(tokens: Vec<Token>) -> Result<Vec<Rc<Box<Node>>>> {
    parse(&mut tokens.iter(), None)
}

fn parse(iter: &mut Iter<Token>, endblock: Option<String>) -> Result<Vec<Rc<Box<Node>>>> {
    let mut root: Vec<Rc<Box<Node>>> = Vec::new();
    let mut cur = iter.next();
    while cur.is_some() {
        let tkn = cur.unwrap();
        match tkn.id {
            TokenId::Text => {
                let tmp = TextNode::new(&tkn.content);
                root.push(Rc::new(Box::new(tmp)));
            },
            TokenId::Variable => {
                if tkn.content.trim().len() > 0 {
                    let tmp = VariableNode::new(tkn);
                    root.push(Rc::new(Box::new(tmp)));
                } else {
                    return Err(Error::new(ErrorKind::InvalidInput, "Empty variable tag"));
                }
            }
            TokenId::Block => {
                let expr = tkn.content.trim();
                if expr.len() > 0 {
                    let (cmd, body) = {
                        let mut split = expr.splitn(2, ' ');
                        (split.next().unwrap().to_string(), split.next().unwrap_or("").to_string())
                    };
                    if endblock == Some(cmd.clone()) {
                        return Ok(root);
                    }
                    match build_expr(cmd, body, iter) {
                        Ok(tmp) => root.push(Rc::new(tmp)),
                        Err(e) => return Err(e),
                    };
                } else {
                    return Err(Error::new(ErrorKind::InvalidInput, "Empty block tag"))
                }
            },
        }
        cur = iter.next();
    }
    Ok(root)
}

use std::convert::AsRef;
fn build_expr(name: String, body: String, iter: &mut Iter<Token>) -> Result<Box<Node>> {
    match name.as_ref() {
        "block" => {
            match parse(iter, Some("endblock".to_string())) {
                Ok(nodes) => {
                    Ok(Box::new(BlockNode::new(body, nodes)))
                },
                Err(e) => Err(e),
            }
        },
        "extends" => {
            Ok(Box::new(ExtendsNode::new(body)))
        },
        _ => Err(Error::new(ErrorKind::Other, format!("Not found tag : {}", name))),
    }
}
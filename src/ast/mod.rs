// The MIT License (MIT)
//
// Copyright (c) 2015 Vladislav Orlov
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

use std::vec::Vec;
use std::boxed::Box;
use std::io::Result;
use std::io::{Error, ErrorKind};
use std::rc::Rc;
use std::slice::Iter;
use mopa::Any;

use scanner::Token;
use scanner::TokenId;
use super::Context;
use tags;

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

pub fn parse(iter: &mut Iter<Token>, endblock: Option<String>) -> Result<Vec<Rc<Box<Node>>>> {
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
                    match tags::build(cmd, body, iter) {
                        Ok(Some(tmp)) => root.push(Rc::new(tmp)),
                        Ok(None) => {},
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

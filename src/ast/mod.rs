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
use std::io::{Result, Error, ErrorKind};
use std::slice::Iter;

pub use scanner::{Token, TokenId};
use super::Context;
use tags;

mod block_node;
mod extends_node;
mod include_node;
mod text_node;
mod variable_node;
mod for_node;
mod if_node;

pub use self::block_node::BlockNode;
pub use self::extends_node::ExtendsNode;
pub use self::include_node::IncludeNode;
pub use self::text_node::TextNode;
pub use self::variable_node::VariableNode;
pub use self::for_node::ForNode;
pub use self::if_node::IfNode;

pub struct ParseResult {
	pub content: Vec<NodeType>,
	pub end_tag: Option<(String, String)>,
}

impl ParseResult {
	fn with_end(c: Vec<NodeType>, end_tag: String, end_tag_body: String) -> ParseResult {
		ParseResult {
			content: c,
			end_tag: Some((end_tag, end_tag_body))
		}
	}
	
	fn without_end(c: Vec<NodeType>) -> ParseResult {
		ParseResult { content: c, end_tag: None }
	}
}

#[derive(Clone)]
pub enum NodeType {
    Block(BlockNode),
    Extends(ExtendsNode),
    Include(IncludeNode),
    Text(TextNode),
    Variable(VariableNode),
    For(ForNode),
    If(IfNode),
}

impl Node for NodeType {
	fn render(&self, c: &Context, s: &mut Vec<String>) -> String {
		match self {
			&NodeType::Block(ref block) => block.render(c, s),
			&NodeType::Extends(ref ext) => ext.render(c, s),
			&NodeType::Include(ref inc) => inc.render(c, s),
			&NodeType::Text(ref text) => text.render(c, s),
			&NodeType::Variable(ref var) => var.render(c, s),
			&NodeType::For(ref for_node) => for_node.render(c, s),
			&NodeType::If(ref if_node) => if_node.render(c, s),
		}
	}
}

pub trait Node {
    fn render(&self, &Context, &mut Vec<String>) -> String;
}

pub fn build(tokens: Vec<Token>) -> Result<Vec<NodeType>> {
	let res = try!(parse(&mut tokens.iter(), Vec::new()));
    Ok(res.content)
}

pub fn parse(iter: &mut Iter<Token>, endblock: Vec<&str>) -> Result<ParseResult> {
    let mut root = Vec::new();
    let mut cur = iter.next();
    while cur.is_some() {
        let tkn = cur.unwrap();
        match tkn.id {
            TokenId::Text => {
                let tmp = TextNode::new(&tkn.content);
                root.push(NodeType::Text(tmp));
            },
            TokenId::Variable => {
                if tkn.content.trim().len() > 0 {
                    let tmp = VariableNode::new(tkn);
                    root.push(NodeType::Variable(tmp));
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
                    if endblock.contains(&cmd.as_ref()) {
                        return Ok(ParseResult::with_end(root, cmd, body));
                    }
                    match tags::build(cmd, body, iter) {
                        Ok(Some(tmp)) => root.push(tmp),
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
    Ok(ParseResult::without_end(root))
}

impl Node for Vec<NodeType> {
	fn render(&self, context: &Context, storage: &mut Vec<String>) -> String {
        let mut res = String::new();
        for node in self.iter() {
            res.push_str(&node.render(context, storage));
        }
        res
	}
}
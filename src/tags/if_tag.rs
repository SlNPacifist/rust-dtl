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

use std::io::{Error, ErrorKind, Result};
use std::slice::Iter;

use ast::{parse, ParseResult, NodeType, IfNode};
use scanner::Token;
use compiler::TemplateCompiler;

pub fn build(body: String, iter: &mut Iter<Token>, compiler: &TemplateCompiler) -> Result<Option<NodeType>> {
	let mut node = IfNode::new();
	let mut cur_condition = Some(body);
	let mut next_condition;
	loop {
		let res = try!(parse(iter, vec!("elif", "else", "endif"), compiler));
		match res {
			ParseResult{content, end_tag: Some((tag, tag_body)) } => {
				let mut should_finish = false;
				next_condition = match tag.as_ref() {
					"elif" => Some(tag_body),
					"else" => None,
					"endif" => {
						should_finish = true;
						None
					}
					_ => None
				};
				match cur_condition {
					Some(cond) => node.add_branch(cond, content),
					None => node.add_else(content),
				};
				if should_finish { break; }
				cur_condition = next_condition;
			}
			_ => { return Err(Error::new(ErrorKind::InvalidInput, "Enclosing endif expected")) }
		}
	}
	Ok(Some(NodeType::If(node)))
}
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

use ast::parse;
use ast::Node;
use ast::ForNode;
use scanner::Token;

fn parse_args(args: String) -> Result<(String, String)> {
	let res : Vec<&str> = args.splitn(2, " in ").collect();
	if res.len() < 2 {
		return Err(Error::new(ErrorKind::InvalidInput, "Tag 'for' requires format 'for var in list'"));
	}
	Ok((res[0].to_string(), res[1].to_string()))
}

pub fn build(body: String, iter: &mut Iter<Token>) -> Result<Option<Box<Node>>> {
	match parse_args(body) {
		Err(e) => Err(e),
		Ok((var_name, list_name)) => {
            match parse(iter, Some("endfor".to_string())) {
                Ok(nodes) => {
                    Ok(Some(Box::new(ForNode::new(var_name, list_name, nodes))))
                },
                Err(e) => Err(e),
            }
		}
	}
}
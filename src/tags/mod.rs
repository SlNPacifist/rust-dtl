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

use std::convert::AsRef;
use std::io::Result;
use std::io::{Error, ErrorKind};
use std::slice::Iter;

use ast::NodeType;
use scanner::Token;

mod block;
mod comment;
mod extends;
mod include;
mod for_tag;

pub fn build(name: String, body: String, iter: &mut Iter<Token>) -> Result<Option<NodeType>> {
    match name.as_ref() {
        "block" => block::build(body, iter),
        "extends" => extends::build(body, iter),
        "include" => include::build(body, iter),
        "comment" => comment::build(body, iter),
        "for" => for_tag::build(body, iter),
        _ => Err(Error::new(ErrorKind::Other, format!("Not found tag : {}", name))),
    }
}

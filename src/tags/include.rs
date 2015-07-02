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

use std::io::Result;
use std::io::{Error, ErrorKind};
use std::slice::Iter;

use Context;
use context::Value;
use ast::Node;
use ast::IncludeNode;
use scanner::Token as ScannerToken;

pub fn build(body: String, _iter: &mut Iter<ScannerToken>) -> Result<Option<Box<Node>>> {
    let mut words = body.split_whitespace();
    let name = try!(words.next().ok_or(Error::new(ErrorKind::Other, "`include` must contain name"))).to_string();

    match words.next() {
        Some("with") => {
            let mut ctx = Context::new();

            loop {
                let expr = words.next();
                if expr.is_none() { break; }
                if expr == Some("only") { break; } //todo:
                let mut expr = expr.unwrap().rsplitn(2, '=');

                let val = expr.next().unwrap();
                let name = expr.next().unwrap();

                println!("{:?} {:?}", name, val);
                if name.len() == 0 {
                    return Err(Error::new(ErrorKind::Other, "`include` has incorrect value (must be op: '=' or name is missing)"));
                }
                ctx.set(&name.to_string(), Box::new(val.to_string()) as Box<Value>);
            }

            Ok(Some(Box::new(IncludeNode::new(name, Some(ctx)))))
        },
        Some("only") => Ok(Some(Box::new(IncludeNode::new(name, Some(Context::new()))))),
        Some(_) => Err(Error::new(ErrorKind::Other, "`include` has incorrect value")),
        None => Ok(Some(Box::new(IncludeNode::new(name, None)))),
    }
}

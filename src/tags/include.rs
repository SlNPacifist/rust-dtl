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
use std::path::Path;

use context::{Context, HashMapContext};
use ast::{NodeType, IncludeNode};
use scanner::Token as ScannerToken;
use compiler::TemplateCompiler;

fn get_template_ast(s: &str, compiler: &TemplateCompiler) -> Result<Vec<NodeType>>{
    let mut count = 0;
    for ch in s.chars() {
        if ch == '"' {
            count += 1;
        }
    }
    let name = try!(match count {
    	0 => Ok(s.to_string()),
    	2 => Ok(s.trim_matches('"').to_string()),
    	_ => Err(Error::new(ErrorKind::Other, format!("Wrong template name in include: {}", s))),
    });
    let template = try!(compiler.compile_file(Path::new(&name)));
    Ok(template.ast)
}

pub fn build(body: String, _iter: &mut Iter<ScannerToken>, compiler: &TemplateCompiler) -> Result<Option<NodeType>> {
    let mut words = body.split_whitespace();
    let name = try!(words.next().ok_or(Error::new(ErrorKind::Other, "`include` must contain name"))).to_string();
    let content = try!(get_template_ast(&name, compiler));

    let context = try!(match words.next() {
        Some("with") => {
            let mut ctx = HashMapContext::new();

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
                ctx.set(name, Box::new(val.to_string()));
            }

            Ok(Some(ctx))
        },
        Some("only") => Ok(Some(HashMapContext::new())),
        Some(_) => Err(Error::new(ErrorKind::Other, "`include` has incorrect value")),
        None => Ok(None),
    });
    Ok(Some(NodeType::Include(IncludeNode::new(content, context))))
}

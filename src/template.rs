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

use std::io::Error;
use std::io::ErrorKind;
use std::io::Result;

use context::Context;
use ast::{self, Node, ExtendsNode, NodeType};
use scanner;
use compiler::TemplateCompiler;

pub struct Template {
    pub ast: Vec<NodeType>,
}

impl Template {
    pub fn get_extends_node<'b>(t: &'b Vec<NodeType>) -> Result<Option<&'b ExtendsNode>> {
        let mut is_first = true;
        let mut res = None;
        for node in t.iter() {
        	match node {
        		&NodeType::Extends(ref node) if is_first => res = Some(node),
        		&NodeType::Extends(_) => return Err(Error::new(ErrorKind::Other, "`extends` must be one and at begin!")),
        		_ => {}
        	}
        	is_first = false;
        }
        Ok(res)
    }
    
    fn replace_blocks(org: &mut Vec<NodeType>, rep: &Vec<NodeType>) {
        for node in org.iter_mut() {
        	if let &mut NodeType::Block(ref mut org_block) = node {
				for replace in rep.iter() {
                    if let &NodeType::Block(ref rep_block) = replace {
                    	if org_block.name() == rep_block.name() {
                    		org_block.content = rep_block.content.clone();
                        }
                    }
        		}
        	}
        }
    }
    
    pub fn compile(text: String, compiler: &TemplateCompiler) -> Result<Template> {
        let tokens = scanner::get_tokens(&text).unwrap();
        let t = try!(ast::build(tokens, compiler));
        let is_extends;
        let mut ext_ast = None;
        {
	        is_extends = match try!(Self::get_extends_node(&t)) {
	        	Some(extends_node) => {
	        		let mut parent_template = try!(compiler.compile_file(extends_node.name()));
					Self::replace_blocks(&mut parent_template.ast, &t);
					ext_ast = Some(parent_template.ast);
		            true
	            },
	        	None => false
        	}
        };
        Ok(Template { ast: match is_extends {
        	true => ext_ast.unwrap(),
        	false => t
		}})
    }
    
    pub fn render(&self, context: &Context) -> String {
    	let mut storage = Vec::new();
        self.ast.render(context, &mut storage)
    }
}
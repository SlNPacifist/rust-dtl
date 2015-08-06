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

use std::boxed::Box;
use std::fs;
use std::fs::File;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;
use std::io::Result;
use std::path::Path;
use std::path::PathBuf;

use context::{Context, MultiContext};
use ast::{self, Node, ExtendsNode, NodeType};
use scanner;

pub struct Template {
    ast: Vec<NodeType>,
    path: PathBuf,
    dir: PathBuf,
}

impl Template {
    pub fn new(path: &Path, dir: &Path) -> Self {
        Template {
            ast: Vec::new(),
            path: path.to_path_buf(),
            dir: dir.to_path_buf(),
        }
    }
    fn read_file(&self) -> Result<String> {
        let metadata = try!(fs::metadata(&self.dir));
        if !metadata.is_dir() {
            return Err(Error::new(ErrorKind::InvalidInput, "`dir` is not directory"))
        }
        let mut text = String::new();
        let mut filepath = self.dir.clone();
        filepath.push(self.path.as_path());
        let mut file = try!(File::open(filepath.as_path()));
        try!(file.read_to_string(&mut text));
        Ok(text)
    }
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
    pub fn compile(&mut self) -> Result<()> {
		let text = try!(self.read_file());
        let tokens = scanner::get_tokens(&text).unwrap();
        let t;
        match ast::build(tokens) {
        	Err(e) => return Err(e),
        	Ok(tokens) => t = tokens,
        }
        let is_extends;
        let mut ext_ast = None;
        {
	        is_extends = match try!(Self::get_extends_node(&t)) {
	        	Some(extends_node) => {
		            let mut tpl = Self::new(extends_node.name(), self.dir.as_path());
		            try!(tpl.compile());
					Self::replace_blocks(&mut tpl.ast, &t);
					ext_ast = Some(tpl.ast);
		            true
	            },
	        	None => false
        	}
        };
        self.ast = match is_extends {
        	true => ext_ast.unwrap(),
        	false => t
		};
        Ok(())
    }
}

impl Node for Template {
    fn render(&self, context: &Context) -> String {
    	let mut common_context = MultiContext::new();
    	common_context.add(context);
        common_context.set("___dir", Box::new(self.dir.as_path().to_str().unwrap_or("").to_string()));
        let mut res = String::new();
        for ast in self.ast.iter() {
            res.push_str(&ast.render(&common_context));
        }
        return res;
    }
}
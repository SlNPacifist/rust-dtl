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

use super::{Node, NodeType};
use context::{Context};

#[derive(Clone)]
struct IfBranch {
	condition: String,
	content: Vec<NodeType>,
}

impl IfBranch {
	fn is_true(&self, c: &Context) -> bool {
		match c.get(&self.condition) {
			Some(ref v) => v.as_bool(),
			None => false
		}
	}
}

#[derive(Clone)]
pub struct IfNode {
	branches: Vec<IfBranch>,
	otherwise: Option<Vec<NodeType>>,
}

impl IfNode {
	pub fn new() -> Self {
		IfNode { branches: Vec::new(), otherwise: None }
	}
    pub fn add_branch(&mut self, condition: String, content: Vec<NodeType>) {
    	self.branches.push(IfBranch { condition: condition, content: content });
    }
    pub fn add_else(&mut self, content: Vec<NodeType>) {
    	self.otherwise = Some(content); 
    }
}

impl Node for IfNode {
    fn render(&self, context: &Context, storage: &mut Vec<String>) -> String {
    	for branch in self.branches.iter() {
    		if branch.is_true(context) {
    			return branch.content.render(context, storage)
    		}
    	}
    	match self.otherwise {
    		Some(ref nodes) => nodes.render(context, storage),
    		None => "".to_string(),
    	}
	}
}

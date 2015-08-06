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
use context::{Context, MultiContext};

#[derive(Clone)]
pub struct ForNode {
    var_name: String,
    list_name: String,
    content: Vec<NodeType>,
}

impl ForNode {
    pub fn new(var_name: String, list_name: String, nodes: Vec<NodeType>) -> Self {
        ForNode { var_name: var_name, list_name: list_name, content: nodes }
    }
}

impl Node for ForNode {
    fn render(&self, context: &Context, storage: &mut Vec<String>) -> String {
        let mut res = String::new();
        let children;
        {
	        let var = context.get(&self.list_name);
	        match var {
	        	None => return format!("no_var {}", self.list_name),
	        	Some(t) => children = t.get_iterator(),
	        }
        }
        if let Some(c) = children {
        	let mut combined = MultiContext::new();
        	combined.add(context);
			for child in c {
				combined.set(&self.var_name, child.clone_box());
		        for node in self.content.iter() {
		            res.push_str(&node.render(&combined, storage));
		        }
			}
		}
        res
    }
}

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

use std::rc::Rc;

use super::Node;
use super::NodeType;
use Context;

pub struct BlockNode {
    name: String,
    content: Vec<Rc<Box<Node>>>,
}

impl BlockNode {
    pub fn new(name: String, nodes: Vec<Rc<Box<Node>>>) -> Self {
        BlockNode { name: name, content: nodes }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Node for BlockNode {
    fn node_type(&self) -> NodeType {
        NodeType::Block
    }
    fn render(&self, context: &Context) -> String {
        let mut res = String::new();
        for node in self.content.iter() {
            res.push_str(&node.render(context));
        }
        res
    }
}

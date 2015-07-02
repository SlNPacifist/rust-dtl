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

use scanner::Token;
use filters::FilterExpression;
use super::Node;
use super::NodeType;
use Context;

pub struct VariableNode {
    expr: FilterExpression,
}

impl VariableNode {
    pub fn new(token: &Token) -> VariableNode {
        VariableNode { expr: FilterExpression::new(token) }
    }
}

impl Node for VariableNode {
    fn node_type(&self) -> NodeType {
        NodeType::Variable
    }
    fn print(&self, level: u32) {
        for _ in 0..level {
            print!("  ");
        }
        println!("var: {:?}", self.expr.var());
    }
    fn render(&self, context: &Context) -> String {
        self.expr.render(context)
    }
}
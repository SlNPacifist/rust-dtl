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

use std::collections::HashMap;
use std::io::Result;
use scanner::Token;
use filter::FilterExpression;
use super::Node;
use context::Context;
use filter::FilterFunction;

#[derive(Clone)]
pub struct VariableNode {
    expr: FilterExpression,
}

impl VariableNode {
    pub fn new(token: &Token, filters: &HashMap<String, FilterFunction>) -> Result<VariableNode> {
        Ok(VariableNode { expr: try!(FilterExpression::new(&token.content, filters)) })
    }
}

impl Node for VariableNode {
    fn render(&self, context: &Context, storage: &mut Vec<String>) -> String {
        self.expr.render(context, storage)
    }
}

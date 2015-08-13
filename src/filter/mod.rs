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

use Context;
mod filter_node;
use self::filter_node::FilterNode;
use value::Value;

#[derive(Clone, Debug)]
pub struct FilterExpression {
	var_name: String,
	filters: Vec<FilterNode>,
}

impl FilterExpression {
    pub fn new(expr: &str) -> FilterExpression {
    	let mut expr_splitter = expr.trim().split('|');
    	let var_name = expr_splitter.next().unwrap().to_string();
        FilterExpression {
        	var_name: var_name,
        	filters: expr_splitter.map(FilterNode::from_expression).collect(),
    	}
    }
    
    fn apply_filter(&self, input: Option<Box<Value>>, iterator: &mut Iterator<Item=&FilterNode>, storage: &mut Vec<String>) -> String {
    	match iterator.next() {
    		Some(filter) => {
    			self.apply_filter(filter.apply(input), iterator, storage)
    		},
    		None => {
		    	match input {
		    		None => "".to_string(),
		    		Some(content) => content.as_string_ref(storage).to_string(),
		    	}
    		}
    	}
    }

    pub fn render(&self, context: &Context, storage: &mut Vec<String>) -> String {
    	let val = context.get(&self.var_name);
    	let mut iter = self.filters.iter();
    	let input = match val {
    		Some(content) => Some(content.clone_box()),
    		None => None,
    	};
    	self.apply_filter(input, &mut iter, storage)
    }
}

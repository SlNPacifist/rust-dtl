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

mod filter_node;
mod default_filters;

use std::io::Result;
use std::collections::HashMap;
use self::filter_node::FilterNode;
use value::Value;
use context::Context;
pub use self::filter_node::FilterFunction;
pub use self::default_filters::DEFAULT_FILTERS;
pub use self::default_filters::CHRONO_FILTERS;

#[derive(Clone, Debug)]
pub struct FilterExpression {
	var_name: String,
	filters: Vec<FilterNode>,
}

impl FilterExpression {
    pub fn new(expr: &str, storage: &HashMap<String, FilterFunction>) -> Result<FilterExpression> {
    	let mut expr_splitter = expr.trim().split('|');
    	let var_name = expr_splitter.next().unwrap().to_string();
    	let mut filters = Vec::new();
    	for filter_expr in expr_splitter {
    		let filter = try!(FilterNode::from_expression(filter_expr, storage));
    		filters.push(filter);
    	}
        Ok(FilterExpression {
        	var_name: var_name,
        	filters: filters,
    	})
    }

    pub fn apply(&self, context: &Context) -> Option<Box<Value>> {
    	let val = context.get(&self.var_name);
    	let mut cur_val = match val {
    		Some(content) => Some(content.clone_box()),
    		None => None,
    	};
    	for filter in self.filters.iter() {
    		cur_val = filter.apply(cur_val);
    	}
    	println!("Filter final value is {:?}", cur_val);
    	cur_val
    }
}

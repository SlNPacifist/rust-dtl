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
use std::io::{Result, Error, ErrorKind};
use std::fmt::{self, Debug, Formatter};
use value::Value;

pub type FilterFunction = fn(Option<Box<Value>>, &str) -> Option<Box<Value>>;

pub struct FilterNode {
	func: FilterFunction,
	arg: String,
	name: String,
}

impl Debug for FilterNode {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "FilterNode (name: {}, arg: {})", self.name, self.arg)
	}
}

impl Clone for FilterNode {
	fn clone(&self) -> Self {
		FilterNode {
			func: self.func,
			arg: self.arg.clone(),
			name: self.name.clone(),
		}
	}
}

impl FilterNode {
	pub fn from_expression(expr: &str, filters: &HashMap<String, FilterFunction>) -> Result<FilterNode> {
		let mut part_splitter = expr.splitn(2, ":");
		let name = part_splitter.next().unwrap().to_string();
		match filters.get(&name) {
			Some(filter) => {
				Ok(FilterNode {
					name: name,
					func: *filter,
					arg: match part_splitter.next() {
						Some(args) => args.to_string(),
						None => "".to_string(),
					}
				})
			},
			None => Err(
				Error::new(
					ErrorKind::NotFound,
					format!("No filter with name {}", name)
				)
			)
		}
	}
	
	pub fn apply(&self, input: Option<Box<Value>>) -> Option<Box<Value>> {
		(self.func)(input, &self.arg)
	}
}

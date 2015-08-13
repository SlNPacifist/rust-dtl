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
use value::Value;
use std::collections::HashMap;

fn filter_none(input: Option<Box<Value>>, _: &str) -> Option<Box<Value>> {
	None
}

fn filter_default(input: Option<Box<Value>>, arg: &str) -> Option<Box<Value>> {
	match input {
		Some(Content) => Some(Content),
		None => Some(Box::new(arg.to_string())),
	}
}

type FilterFunction = Fn(Option<Box<Value>>, &str) -> Option<Box<Value>>;
type FilterStorage = HashMap<String, Box<FilterFunction>>;

fn get_global_storage() -> FilterStorage {
	let mut filters: FilterStorage = HashMap::new();
	filters.insert("none".to_string(), Box::new(filter_none));
	filters.insert("default".to_string(), Box::new(filter_default));
	filters
}

#[derive(Clone, Debug)]
pub struct FilterNode {
	name: String,
	arg: String,
}

impl FilterNode {
	pub fn from_expression(expr: &str) -> FilterNode {
		let mut part_splitter = expr.splitn(2, ":");
		FilterNode {
			name: part_splitter.next().unwrap().to_string(),
			arg: match part_splitter.next() {
				Some(args) => args.to_string(),
				None => "".to_string(),
			},
		}
	}
	
	pub fn apply(&self, input: Option<Box<Value>>) -> Option<Box<Value>> {
		let filters = get_global_storage();
		match filters.get(&self.name) {
			Some(filter) => {
				filter(input, &self.arg)
			},
			None => panic!(format!("No filter with name {}", self.name))
		}
	}
}

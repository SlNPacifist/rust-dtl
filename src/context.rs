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
use std::collections::HashMap;
use value::Value;
use std::ops::Deref;

pub trait Context {
    fn get(&self, field: &str) -> Option<&Value>;
    fn set(&mut self, field: &str, value: Box<Value>);
}

#[derive(Clone, Debug)]
pub struct HashMapContext {
    dict: HashMap<String, Box<Value>>,
}

impl HashMapContext {
    pub fn new() -> Self {
        HashMapContext { dict: HashMap::new() }
    }
}

impl Context for HashMapContext {
    fn get(&self, field: &str) -> Option<&Value> {
    	{
	        let res = self.dict.get(field);
	        if let Some(a) = res { return Some(a.deref()) }
        }
		let mut splitter = field.split('.');
		if let Some(var) = self.dict.get(splitter.next().unwrap()) {
			if let Some(part) = splitter.next() {
    			if let Some(mut prop) = var.get_property(part) {
	        		while let Some(part) = splitter.next() {
	        			match prop.get_property(part) {
	        				None => { return None; },
	        				Some(tmp) => { prop = tmp; }
	        			}
	        		}
	        		return Some(prop);
    			}
			}
		}
		None
    }

    fn set(&mut self, field: &str, value: Box<Value>) {
        self.dict.insert(field.to_string(), value);
    }
}


pub struct MultiContext<'a> {
	default_context: HashMapContext,
	contexts: Vec<&'a Context>,
}

impl<'a> MultiContext<'a> {
	pub fn new() -> MultiContext<'a> {
		MultiContext {
			default_context: HashMapContext::new(),
			contexts: Vec::new(),
		}
	}

	pub fn add(&mut self, c: &'a Context) {
		self.contexts.push(c);
	}
}

impl<'a> Context for MultiContext<'a> {
    fn get(&self, field: &str) -> Option<&Value> {
		if let Some(v) = self.default_context.get(field) {
			return Some(v)
		}
    	for c in self.contexts.iter() {
    		if let Some(v) = c.get(field) {
    			return Some(v)
    		}
    	}
    	None
    }
    
    fn set(&mut self, field: &str, value: Box<Value>) {
    	self.default_context.set(field, value);
    }
}

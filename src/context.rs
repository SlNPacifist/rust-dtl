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

use std::any::Any;
use std::boxed::Box;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Display;

pub trait Value: Any + Display + Debug + ValueClone {
	fn get_children(&self) -> Vec<Box<Value>>;
	fn get_property(&self, name: &str) -> Option<Box<Value>>;
}

trait ValueClone {
    fn clone_box(&self) -> Box<Value>;
}

impl<T> ValueClone for T where T: 'static + Value + Clone {
    fn clone_box(&self) -> Box<Value> {
        Box::new(self.clone())
    }
}

impl Clone for Box<Value> {
    fn clone(&self) -> Box<Value> {
        self.clone_box()
    }
}

impl Value for String {
	fn get_children(&self) -> Vec<Box<Value>> {
		Vec::new()
	}
	
	fn get_property(&self, name: &str) -> Option<Box<Value>> {
		None
	}
}

#[derive(Clone, Debug)]
pub struct Context {
    dict: HashMap<String, Box<Value>>,
}

impl Context {
    pub fn new() -> Self {
        let dict = HashMap::new();
        Context { dict: dict }
    }
    pub fn get(&mut self, field: &str) -> Option<&Box<Value>> {
    	{
	        let res = self.dict.get(field);
	        if let Some(a) = res { return res }
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

    pub fn set(&mut self, field: &str, value: Box<Value>) {
        self.dict.insert(field.to_string(), value);
    }
}

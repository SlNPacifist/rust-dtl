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

use super::Node;
use super::NodeType;
use std::path::Path;
use Context;

pub struct ExtendsNode {
    name: String,
    dynamic: bool
}

impl ExtendsNode {
    pub fn new(name: String) -> ExtendsNode {
        let s = name.trim();
        let mut count = 0;
        for ch in s.chars() {
            if ch == '"' {
                count += 1;
            }
        }
        if count == 0 {
            ExtendsNode { name: s.to_string(), dynamic: true }
        } else {
            if count != 2 {
                panic!("Oops! Need correct name"); // FIXME: asd
            } else {
                ExtendsNode { name: s.trim_matches('"').to_string(), dynamic: false }
            }
        }
    }

    pub fn name(&self) -> &Path {
        if !self.dynamic {
            Path::new(&self.name)
        } else {
            unimplemented!();
        }
    }
}

impl Node for ExtendsNode {
    fn node_type(&self) -> NodeType {
        NodeType::Extends
    }
    fn render(&self, _context: &mut Context) -> String {
        "".to_string()
    }
}

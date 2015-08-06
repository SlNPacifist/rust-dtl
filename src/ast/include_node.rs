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
use std::path::Path;
use context::{HashMapContext, Context};
use Template;

#[derive(Clone)]
pub struct IncludeNode {
    name: String,
    context: Option<HashMapContext>,
    dynamic: bool,
}

impl IncludeNode {
    pub fn new(name: String, ctx: Option<HashMapContext>) -> IncludeNode {
        let s = name.trim();
        let mut count = 0;
        for ch in s.chars() {
            if ch == '"' {
                count += 1;
            }
        }
        if count == 0 {
            IncludeNode { name: s.to_string(), context: ctx, dynamic: true }
        } else {
            if count != 2 {
                panic!("Oops! Need correct name"); 
            } else {
                IncludeNode { name: s.trim_matches('"').to_string(), context: ctx, dynamic: false }
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

impl Node for IncludeNode {
    fn render(&self, ctx: &Context) -> String {
        let mut tpl = Template::new(Path::new(self.name()), Path::new(ctx.get("___dir").unwrap().as_string_ref()));
        match tpl.compile() {
            Ok(_) => {},
            Err(e) => panic!(e),
        };
        // TODO: merge `context` and `self.context`
        tpl.render(ctx)
    }
}

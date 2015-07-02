//! Compile Django Template file as byte-code, and then we can render them
//!

#[macro_use]
extern crate mopa;

use std::fs::File;
use std::io::Read;
use std::io::Result;
use std::io::Error;
use std::io::ErrorKind;
use std::collections::HashMap;
use std::fmt::Display;
use std::path::Path;
use std::path::PathBuf;
use std::boxed::Box;
use std::rc::Rc;
use std::fs;

mod ast;
mod scanner;
mod filters;

use ast::Node;
use ast::ExtendsNode;
use ast::BlockNode;

pub struct Template {
    ast: Vec<Rc<Box<Node>>>,
    path: PathBuf,
    dir: PathBuf,
}

impl Template {
    pub fn new(path: &Path, dir: &Path) -> Self {
        Template {
            ast: Vec::new(),
            path: path.to_path_buf(),
            dir: dir.to_path_buf(),
        }
    }
    pub fn compile(&mut self) -> Result<()> {
        let metadata = try!(fs::metadata(&self.dir));
        if !metadata.is_dir() {
            return Err(Error::new(ErrorKind::InvalidInput, "`dir` is not directory"))
        }
        let mut text = String::new();

        let mut filepath = self.dir.clone();
        filepath.push(self.path.as_path());
        let mut file = try!(File::open(filepath.as_path()));
        try!(file.read_to_string(&mut text));

        let tokens = scanner::get_tokens(&text).unwrap();
        // println!("TOKENS: {:?}", tokens);

        match ast::build(tokens) {
            Ok(t) => {
                let new = {
                    use ast::NodeType;
                    let is_extends = {
                        let mut iter = t.iter();
                        let is_extends = match iter.next() {
                            Some(node) if node.node_type() == NodeType::Extends => Some(node),
                            _ => None,
                        };
                        for ast in iter {
                            if ast.node_type() == NodeType::Extends {
                                return Err(Error::new(ErrorKind::Other, "`extends` must be one and at begin!"));
                            }
                            ast.print(0);
                        }
                        is_extends
                    };

                    // merge
                    match is_extends {
                        Some(ext) => {
                            let ext = ext.downcast_ref::<ExtendsNode>();
                            let ext = match ext {
                                Some(v) => v,
                                None => return Err(Error::new(ErrorKind::Other, "`extends` couldn't downcast!")),
                            };
                            println!("test {:?} {:?}", Path::new(ext.name()), self.dir.as_path());
                            let mut tpl = Self::new(ext.name(), self.dir.as_path());
                            match tpl.compile() {
                                Err(e) => return Err(e),
                                _ => {},
                            }
                            println!("=== MERGE");
                            for node in tpl.ast.iter_mut() {
                                if node.node_type() == NodeType::Block {
                                    for replace in t.iter() {
                                        if replace.node_type() == NodeType::Block {
                                            let (t1, t2) = {
                                                let t1 = replace.downcast_ref::<BlockNode>();
                                                let t2 = node.downcast_ref::<BlockNode>();
                                                let t1 = match t1 {
                                                    Some(v) => v,
                                                    None => return Err(Error::new(ErrorKind::Other, "`block` couldn't downcast!")),
                                                };
                                                let t2 = match t2 {
                                                    Some(v) => v,
                                                    None => return Err(Error::new(ErrorKind::Other, "`block` couldn't downcast!")),
                                                };
                                                (t1.name().to_string(), t2.name().to_string())
                                            };
                                            if t1 == t2 {
                                                node.clone_from(replace);
                                            }
                                        }
                                    }
                                }
                                node.print(0);
                            }

                            Some(tpl.ast)
                        },
                        None => None,
                    }
                };

                self.ast = match new {
                    Some(ast) => ast,
                    None => t,
                };

                Ok(())
            },
            Err(e) => Err(e),
        }
    }

    pub fn render(&self, context: &Context) -> String {
        let mut res = String::new();
        for ast in self.ast.iter() {
            res.push_str(&ast.render(&context));
            println!("#{:?}", ast.render(&context));
        }
        return res;

    }

    pub fn print(&self) {
        println!("=== PRINT AST:");
        for node in self.ast.iter() {
            node.print(0);
        }
    }
}

pub struct Context {
    dict: HashMap<String, Box<Display>>,
}

impl Context {
    pub fn new() -> Self {
        let dict = HashMap::new();
        Context { dict: dict }
    }
    pub fn get(&self, field: &str) -> Option<&Box<Display>> {
        self.dict.get(field)
    }

    pub fn set(&mut self, field: &str, value: Box<Display>) {
        self.dict.insert(field.to_string(), value);
    }
}

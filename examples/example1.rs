extern crate dtl;

use std::path::{Path, PathBuf};
use dtl::{TemplateCompiler, Context, HashMapContext};

fn main() {
    let mut ctx = HashMapContext::new();
    ctx.set("username", Box::new("Ivan Ivanov".to_string()));
    ctx.set("test_var", Box::new("test-barstring".to_string()));
    let root = PathBuf::from("examples/views/");
    let res = TemplateCompiler::render_file(root, Path::new("welcome.html"), &ctx);
    println!("{}", res.unwrap());
}

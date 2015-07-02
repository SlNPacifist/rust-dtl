extern crate dtl;

use std::path::Path;
use std::error::Error;
use dtl::{Context, Template};

fn main() {
    let mut ctx = Context::new();
    ctx.set("username", Box::new("Ivan Ivanov".to_string()));
    ctx.set("test_var", Box::new("test-barstring".to_string()));
    let mut tpl = Template::new(Path::new("welcome.html"), Path::new("examples/views/"));
    match tpl.compile() {
        Ok(_) => {},
        Err(e) => panic!("{}", e),
    };
    println!("{}", tpl.render(&mut ctx));
}

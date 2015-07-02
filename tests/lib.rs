use std::fs::File;
use std::io::Read;
use std::path::Path;

extern crate dtl;
use dtl::Context;
use dtl::Template;

#[test]
fn comment() {
    let mut except = String::new();
    let file = File::open("tests/files/except/comment");
    assert!(file.is_ok());
    assert!(file.unwrap().read_to_string(&mut except).is_ok());
    let ctx = Context::new();
    let mut tpl = Template::new(Path::new("comment"), Path::new("tests/files/input/"));
    match tpl.compile() {
        Err(e) => panic!(format!("{}", e)),
        _ => {}
    }
    assert_eq!(tpl.render(&ctx), except);
}

#[test]
fn extends() {
    let mut except = String::new();
    let file = File::open("tests/files/except/extends");
    assert!(file.is_ok());
    assert!(file.unwrap().read_to_string(&mut except).is_ok());
    let mut ctx = Context::new();
    ctx.set("base_var", Box::new("base-barstring"));
    ctx.set("test_var", Box::new("test-barstring"));
    let mut tpl = Template::new(Path::new("extends"), Path::new("tests/files/input/"));
    match tpl.compile() {
        Err(e) => panic!(format!("{}", e)),
        _ => {}
    }
    assert_eq!(tpl.render(&ctx), except);
}

#[test]
fn extends_4() {
    let mut except = String::new();
    let file = File::open("tests/files/except/extends4");
    assert!(file.is_ok());
    assert!(file.unwrap().read_to_string(&mut except).is_ok());
    let ctx = Context::new();
    let mut tpl = Template::new(Path::new("extends4"), Path::new("tests/files/input/"));
    match tpl.compile() {
        Err(e) => panic!(format!("{}", e)),
        _ => {}
    }
    assert_eq!(tpl.render(&ctx), except);
}

#[test]
fn extends_recursive_block() {
    let mut except = String::new();
    let file = File::open("tests/files/except/extends_recursive_block");
    assert!(file.is_ok());
    assert!(file.unwrap().read_to_string(&mut except).is_ok());
    let ctx = Context::new();
    let mut tpl = Template::new(Path::new("extends_recursive_block"), Path::new("tests/files/input/"));
    match tpl.compile() {
        Err(e) => panic!(format!("{}", e)),
        _ => {}
    }
    assert_eq!(tpl.render(&ctx), except);
}

#[test]
fn include() {
    let mut except = String::new();
    let file = File::open("tests/files/except/include");
    assert!(file.is_ok());
    assert!(file.unwrap().read_to_string(&mut except).is_ok());
    let mut ctx = Context::new();
    ctx.set("var1", Box::new("foostring1"));
    let mut tpl = Template::new(Path::new("include"), Path::new("tests/files/input/"));
    match tpl.compile() {
        Err(e) => panic!(format!("{}", e)),
        _ => {}
    }
    assert_eq!(tpl.render(&ctx), except);
}
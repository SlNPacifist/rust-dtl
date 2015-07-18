use std::fs::File;
use std::io::{Read, Write};
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
    let mut ctx = Context::new();
    let mut tpl = Template::new(Path::new("comment"), Path::new("tests/files/input/"));
    match tpl.compile() {
        Err(e) => panic!(format!("{}", e)),
        _ => {}
    }
    assert_eq!(tpl.render(&mut ctx), except);
}

#[test]
fn extends() {
    let mut except = String::new();
    let file = File::open("tests/files/except/extends");
    assert!(file.is_ok());
    assert!(file.unwrap().read_to_string(&mut except).is_ok());
    let mut ctx = Context::new();
    ctx.set("base_var", Box::new("base-barstring".to_string()));
    ctx.set("test_var", Box::new("test-barstring".to_string()));
    let mut tpl = Template::new(Path::new("extends"), Path::new("tests/files/input/"));
    match tpl.compile() {
        Err(e) => panic!(format!("{}", e)),
        _ => {}
    }
    assert_eq!(tpl.render(&mut ctx), except);
}

#[test]
fn extends4() {
    let mut except = String::new();
    let file = File::open("tests/files/except/extends4");
    assert!(file.is_ok());
    assert!(file.unwrap().read_to_string(&mut except).is_ok());
    let mut ctx = Context::new();
    let mut tpl = Template::new(Path::new("extends4"), Path::new("tests/files/input/"));
    match tpl.compile() {
        Err(e) => panic!(format!("{}", e)),
        _ => {}
    }
    assert_eq!(tpl.render(&mut ctx), except);
}

#[test]
fn extends_path() {
    let mut except = String::new();
    let file = File::open("tests/files/except/extends_path");
    assert!(file.is_ok());
    assert!(file.unwrap().read_to_string(&mut except).is_ok());
    let mut ctx = Context::new();
    ctx.set("base_var", Box::new("base-barstring".to_string()));
    ctx.set("test_var", Box::new("test-barstring".to_string()));
    let mut tpl = Template::new(Path::new("extends_path"), Path::new("tests/files/input/"));
    match tpl.compile() {
        Err(e) => panic!(format!("{}", e)),
        _ => {}
    }
    assert_eq!(tpl.render(&mut ctx), except);
}

#[test]
fn extends_path2() {
    let mut except = String::new();
    let file = File::open("tests/files/except/extends_path2");
    assert!(file.is_ok());
    assert!(file.unwrap().read_to_string(&mut except).is_ok());
    let mut ctx = Context::new();
    let mut tpl = Template::new(Path::new("extends_path2"), Path::new("tests/files/input/"));
    match tpl.compile() {
        Err(e) => panic!(format!("{}", e)),
        _ => {}
    }
    assert_eq!(tpl.render(&mut ctx), except);
}

#[test]
fn extends_recursive_block() {
    let mut except = String::new();
    let file = File::open("tests/files/except/extends_recursive_block");
    assert!(file.is_ok());
    assert!(file.unwrap().read_to_string(&mut except).is_ok());
    let mut ctx = Context::new();
    let mut tpl = Template::new(Path::new("extends_recursive_block"), Path::new("tests/files/input/"));
    match tpl.compile() {
        Err(e) => panic!(format!("{}", e)),
        _ => {}
    }
    assert_eq!(tpl.render(&mut ctx), except);
}

#[test]
fn include() {
    let mut except = String::new();
    let file = File::open("tests/files/except/include");
    assert!(file.is_ok());
    assert!(file.unwrap().read_to_string(&mut except).is_ok());
    let mut ctx = Context::new();
    ctx.set("var1", Box::new("foostring1".to_string()));
    let mut tpl = Template::new(Path::new("include"), Path::new("tests/files/input/"));
    match tpl.compile() {
        Err(e) => panic!(format!("{}", e)),
        _ => {}
    }
    assert_eq!(tpl.render(&mut ctx), except);
}

#[test]
fn include_template() {
    let mut except = String::new();
    let file = File::open("tests/files/except/include_template");
    assert!(file.is_ok());
    assert!(file.unwrap().read_to_string(&mut except).is_ok());
    let mut ctx = Context::new();
    ctx.set("base_var", Box::new("base-barstring".to_string()));
    ctx.set("test_var", Box::new("test-barstring".to_string()));
    let mut tpl = Template::new(Path::new("include_template"), Path::new("tests/files/input/"));
    match tpl.compile() {
        Err(e) => panic!(format!("{}", e)),
        _ => {}
    }
    assert_eq!(tpl.render(&mut ctx), except);
}

#[test]
fn path1() {
    let mut except = String::new();
    let file = File::open("tests/files/except/path1");
    assert!(file.is_ok());
    assert!(file.unwrap().read_to_string(&mut except).is_ok());
    let mut ctx = Context::new();
    let mut tpl = Template::new(Path::new("path1/base1"), Path::new("tests/files/input/"));
    match tpl.compile() {
        Err(e) => panic!(format!("{}", e)),
        _ => {}
    }
    assert_eq!(tpl.render(&mut ctx), except);
}

#[test]
fn for_test() {
    let mut ctx = Context::new();
    ctx.set("a", Box::new("base-barstring".to_string()));
    let mut tpl = Template::new(Path::new("for"), Path::new("tests/files/input/"));
    match tpl.compile() {
        Err(e) => panic!(format!("{}", e)),
        _ => {}
    }
    std::io::stdout().write(&tpl.render(&mut ctx).into_bytes());
}
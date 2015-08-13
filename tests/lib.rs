use std::fs::File;
use std::io::Read;
use std::path::Path;

extern crate dtl;
use dtl::{Template, Context, HashMapContext};

mod vec_of_strings;
use vec_of_strings::VecOfStrings;


#[test]
fn comment() {
    let mut except = String::new();
    let file = File::open("tests/files/except/comment");
    assert!(file.is_ok());
    assert!(file.unwrap().read_to_string(&mut except).is_ok());
    let mut ctx = HashMapContext::new();
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
    let mut ctx = HashMapContext::new();
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
    let mut ctx = HashMapContext::new();
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
    let mut ctx = HashMapContext::new();
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
    let mut ctx = HashMapContext::new();
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
    let mut ctx = HashMapContext::new();
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
    let mut ctx = HashMapContext::new();
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
    let mut ctx = HashMapContext::new();
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
    let mut ctx = HashMapContext::new();
    let mut tpl = Template::new(Path::new("path1/base1"), Path::new("tests/files/input/"));
    match tpl.compile() {
        Err(e) => panic!(format!("{}", e)),
        _ => {}
    }
    assert_eq!(tpl.render(&mut ctx), except);
}

#[test]
fn for_test() {
    let mut except = String::new();
    let file = File::open("tests/files/except/for");
    assert!(file.is_ok());
    assert!(file.unwrap().read_to_string(&mut except).is_ok());
    let mut ctx = HashMapContext::new();
	let strings = VecOfStrings::new(vec!("first", "second", "third"));
    ctx.set("b", Box::new(strings));
    let mut tpl = Template::new(Path::new("for"), Path::new("tests/files/input/"));
    match tpl.compile() {
        Err(e) => panic!(format!("{}", e)),
        _ => {}
    }
    assert_eq!(tpl.render(&mut ctx), except);
}

#[test]
fn if_test() {
    let first_true = "\ncondition is true!\n";
    let second_true = "\ncondition2 is true!\n";
    let none_true = "\nnothing is true!\n";
    let samples = vec!(
		(Some(true), Some(false), first_true),
		(Some(true), Some(true), first_true),
		(None, Some(true), second_true),
		(Some(false), None, none_true),
		(None, None, none_true),
	);
    let mut tpl = Template::new(Path::new("if"), Path::new("tests/files/input/"));
    tpl.compile().unwrap();
    for sample in samples.iter() {
    	let (cond1, cond2, res) = *sample;
    	let mut ctx = HashMapContext::new();
    	if cond1.is_some() { ctx.set("condition", Box::new(cond1.unwrap())) };
    	if cond2.is_some() { ctx.set("condition2", Box::new(cond2.unwrap())) };
    	assert_eq!(tpl.render(&ctx), res);
    }
}

use std::io::stderr;
use std::io::Write;
#[test]
fn filter_test() {
    let mut tpl = Template::new(Path::new("filter"), Path::new("tests/files/input/"));
    tpl.compile().unwrap();
	let mut ctx = HashMapContext::new();
	ctx.set("a", Box::new(2));
	ctx.set("b", Box::new("abc".to_string()));
	stderr().write_all(tpl.render(&ctx).as_bytes());
}
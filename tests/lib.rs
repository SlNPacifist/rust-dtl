extern crate dtl;
mod vec_of_strings;
mod filter_const;

use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use dtl::{TemplateCompiler, Context, HashMapContext};
use vec_of_strings::VecOfStrings;
use filter_const::filter_const;

static TEMPLATE_ROOT: &'static str = "tests/files/input/";
static EXPECTED_ROOT: &'static str = "tests/files/except/";

fn read_file(name: &str) -> String {
    let mut res = String::new();
    let file = File::open(name);
    assert!(file.is_ok());
    assert!(file.unwrap().read_to_string(&mut res).is_ok());
    res
}

fn render_check(template_name: &str, context: &Context, expected_file_name: &str) {
	let mut expected_name = EXPECTED_ROOT.to_string();
	expected_name.push_str(&expected_file_name);
	let expected = read_file(&expected_name);
	let root = PathBuf::from(TEMPLATE_ROOT);
	let res = TemplateCompiler::render_file(root, Path::new(template_name), context);
	assert!(res.is_ok());
	assert_eq!(res.unwrap(), expected);
}

#[test]
fn comment() {
    let ctx = HashMapContext::new();
    render_check("comment", &ctx, "comment");
}

#[test]
fn extends() {
    let mut ctx = HashMapContext::new();
    ctx.set("base_var", Box::new("base-barstring".to_string()));
    ctx.set("test_var", Box::new("test-barstring".to_string()));
	render_check("extends", &ctx, "extends");
}

#[test]
fn extends4() {
    let ctx = HashMapContext::new();
	render_check("extends4", &ctx, "extends4");
}

#[test]
fn extends_path() {
    let mut ctx = HashMapContext::new();
    ctx.set("base_var", Box::new("base-barstring".to_string()));
    ctx.set("test_var", Box::new("test-barstring".to_string()));
	render_check("extends_path", &ctx, "extends_path");
}

#[test]
fn extends_path2() {
    let ctx = HashMapContext::new();
    render_check("extends_path2", &ctx, "extends_path2");
}

#[test]
fn extends_recursive_block() {
    let ctx = HashMapContext::new();
    render_check("extends_recursive_block", &ctx, "extends_recursive_block");
}

#[test]
fn include() {
    let mut ctx = HashMapContext::new();
    ctx.set("var1", Box::new("foostring1".to_string()));
    render_check("include", &ctx, "include");
}

#[test]
fn include_template() {
    let mut ctx = HashMapContext::new();
    ctx.set("base_var", Box::new("base-barstring".to_string()));
    ctx.set("test_var", Box::new("test-barstring".to_string()));
    render_check("include_template", &ctx, "include_template");
}

#[test]
fn path1() {
    let ctx = HashMapContext::new();
    render_check("path1/base1", &ctx, "path1");
}

#[test]
fn for_test() {
    let mut ctx = HashMapContext::new();
	let strings = VecOfStrings::new(vec!("first", "second", "third"));
    ctx.set("b", Box::new(strings));
	render_check("for", &ctx, "for");
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
    let root = PathBuf::from(TEMPLATE_ROOT);
    let compiler = TemplateCompiler::new(root).unwrap();
	let tpl = compiler.compile_file(Path::new("if")).unwrap();
    for sample in samples.iter() {
    	let (cond1, cond2, res) = *sample;
    	let mut ctx = HashMapContext::new();
    	if cond1.is_some() { ctx.set("condition", Box::new(cond1.unwrap())) };
    	if cond2.is_some() { ctx.set("condition2", Box::new(cond2.unwrap())) };
    	assert_eq!(tpl.render(&ctx), res);
    }
}

#[test]
fn filter_test() {
	let mut ctx = HashMapContext::new();
	ctx.set("a", Box::new(2));
	ctx.set("b", Box::new("abc".to_string()));
	render_check("filter", &ctx, "filter");
}

#[test]
fn custom_filter_test() {
	let ctx = HashMapContext::new();
    let root = PathBuf::from(TEMPLATE_ROOT);
    let mut compiler = TemplateCompiler::new(root).unwrap();
	compiler.add_filter("const".to_string(), filter_const);
	let tpl = compiler.compile_file(Path::new("custom_filter")).unwrap();
	assert_eq!(tpl.render(&ctx), "abc");
}
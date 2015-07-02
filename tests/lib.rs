use std::fs::File;
use std::io::Read;
use std::path::Path;

extern crate dtl;
use dtl::Context;
use dtl::Template;

#[test]
fn extend_1() {
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
	tpl.print();
	assert_eq!(tpl.render(&ctx), except);
}
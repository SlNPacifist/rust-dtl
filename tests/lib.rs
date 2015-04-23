use std::fs::File;
use std::io::Read;

extern crate dtl;
use dtl::DTL;

#[test]
fn extend_1() {
	let mut str = String::new();
	let _ = File::open("tests/files/except/extends").unwrap().read_to_string(&mut str);
	assert_eq!(DTL::compile_file("tests/files/input/extends"), str);
}
extern crate dtl;
use dtl::Value;

pub fn filter_const(_: Option<Box<Value>>, arg: &str) -> Option<Box<Value>> {
	Some(Box::new(arg.to_string()))
}
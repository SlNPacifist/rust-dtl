extern crate dtl;
use dtl::{Value, ValueAsStringByRef, ValueAsIterator, ValueAsObject};

#[derive(Debug, Clone)]
pub struct VecOfStrings {
	pub children: Vec<String> 
}

impl VecOfStrings {
	pub fn new(v: Vec<&str>) -> VecOfStrings {
		let cloner = |x: &&str| x.to_string();
		let c = v.iter().map(cloner).collect();
		VecOfStrings {
			children: c
		}
	}
}

fn string_to_value(s: &String) -> &Value {
	s
}

impl ValueAsStringByRef for VecOfStrings {
	fn as_string_ref(&self) -> &str {
		"none"
	}
}

impl ValueAsIterator for VecOfStrings {
	fn get_iterator<'a>(&'a self) -> Option<Box<Iterator<Item=&Value> + 'a>> {
		Some(Box::new(self.children.iter().map(string_to_value)))
	} 
}

impl ValueAsObject for VecOfStrings {
	fn get_property(&self, _: &str) -> Option<&Value> {
		None
	}
}

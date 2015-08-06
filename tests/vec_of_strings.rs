extern crate dtl;
use dtl::{Value, ValueAsString, ValueAsIterator, ValueAsObject};

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

impl ValueAsString for VecOfStrings {
	fn as_string(&self) -> String {
		format!("VecOfStrings({} elements)", self.children.len())
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

extern crate dtl;
use dtl::Value;
use std::fmt;

#[derive(Clone, Debug)]
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

impl fmt::Display for VecOfStrings {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VecOfStrings({})", self.children.len())
    }
}

fn box_string(s: &String) -> Box<Value> {
	Box::new(s.clone())
}

impl Value for VecOfStrings {
	fn get_children(&self) -> Vec<Box<Value>> {
		self.children.iter().map(box_string).collect()
	} 
}
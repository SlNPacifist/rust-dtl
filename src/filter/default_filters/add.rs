use value::Value;

pub fn apply(input: Option<Box<Value>>, _: &str) -> Option<Box<Value>> {
	match input {
		Some(content) => {
			match content.downcast_ref::<i32>() {
				Some(val) => Some(Box::new(val + 7)),
				None => None
			}
		},
		None => None
	}
}
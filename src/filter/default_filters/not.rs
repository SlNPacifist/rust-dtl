use value::Value;

pub fn apply(input: Option<Box<Value>>, _: &str) -> Option<Box<Value>> {
	match input {
		Some(val) => Some(Box::new(!val.as_bool())),
		None => Some(Box::new(true))
	}
	
}
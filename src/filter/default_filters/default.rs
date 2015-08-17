use value::Value;

pub fn apply(input: Option<Box<Value>>, arg: &str) -> Option<Box<Value>> {
	match input {
		Some(content) => Some(content),
		None => Some(Box::new(arg.to_string())),
	}
}
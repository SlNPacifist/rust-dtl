use value::{Value, ValueAsString, ValueAsIterator, ValueAsObject, ValueAsBool};

impl ValueAsString for f32 {
	fn as_string(&self) -> String {
		format!("{}", &self)
	}
}

impl ValueAsIterator for f32 {
	fn get_iterator(&self) -> Option<Box<Iterator<Item=&Value>>> {
		None
	}
}

impl ValueAsObject for f32 {
	fn get_property(&self, _: &str) -> Option<&Value> {
		None
	}
}

impl ValueAsBool for f32 {
	fn as_bool(&self) -> bool {
		false
	}
}
use value::{Value, ValueAsString, ValueAsIterator, ValueAsObject, ValueAsBool};

impl ValueAsString for i32 {
	fn as_string(&self) -> String {
		format!("{}", &self)
	}
}

impl ValueAsIterator for i32 {
	fn get_iterator(&self) -> Option<Box<Iterator<Item=&Value>>> {
		None
	}
}

impl ValueAsObject for i32 {
	fn get_property(&self, _: &str) -> Option<&Value> {
		None
	}
}

impl ValueAsBool for i32 {
	fn as_bool(&self) -> bool {
		*self != 0
	}
}
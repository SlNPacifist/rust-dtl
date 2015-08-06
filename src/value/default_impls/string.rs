use value::{Value, ValueAsStringByRef, ValueAsIterator, ValueAsObject, ValueAsBool};

impl ValueAsStringByRef for String {
	fn as_string_ref(&self, _: &mut Vec<String>) -> &str {
		&self
	}
}

impl ValueAsIterator for String {
	fn get_iterator(&self) -> Option<Box<Iterator<Item=&Value>>> {
		None
	}
}

impl ValueAsObject for String {
	fn get_property(&self, _: &str) -> Option<&Value> {
		None
	}
}

impl ValueAsBool for String {
	fn as_bool(&self) -> bool {
		self.len() > 0
	}
}
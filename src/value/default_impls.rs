use value::{Value, ValueAsStringByRef, ValueAsIterator, ValueAsObject};

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

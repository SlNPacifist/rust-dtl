use value::{Value, ValueAsStringByRef, ValueAsIterator, ValueAsObject, ValueAsBool};

impl ValueAsStringByRef for bool {
	fn as_string_ref(&self, _: &mut Vec<String>) -> &str {
		match *self {
			true => "true",
			false => "false",
		}
	}
}

impl ValueAsIterator for bool {
	fn get_iterator(&self) -> Option<Box<Iterator<Item=&Value>>> {
		None
	}
}

impl ValueAsObject for bool {
	fn get_property(&self, _: &str) -> Option<&Value> {
		None
	}
}

impl ValueAsBool for bool {
	fn as_bool(&self) -> bool {
		*self
	}
}
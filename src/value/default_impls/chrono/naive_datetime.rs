use super::chrono::NaiveDateTime;
use value::{Value, ValueAsString, ValueAsIterator, ValueAsObject, ValueAsBool};

impl ValueAsString for NaiveDateTime {
	fn as_string(&self) -> String {
		format!("{}", self)
	}
}

impl ValueAsIterator for NaiveDateTime {
	fn get_iterator(&self) -> Option<Box<Iterator<Item=&Value>>> {
		None
	}
}

impl ValueAsObject for NaiveDateTime {
	fn get_property(&self, _: &str) -> Option<&Value> {
		None
	}
}

impl ValueAsBool for NaiveDateTime {
	fn as_bool(&self) -> bool {
		true
	}
}
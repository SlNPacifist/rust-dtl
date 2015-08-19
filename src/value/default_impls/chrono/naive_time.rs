use super::chrono::NaiveTime;
use value::{Value, ValueAsString, ValueAsIterator, ValueAsObject, ValueAsBool};

impl ValueAsString for NaiveTime {
	fn as_string(&self) -> String {
		format!("{}", self)
	}
}

impl ValueAsIterator for NaiveTime {
	fn get_iterator(&self) -> Option<Box<Iterator<Item=&Value>>> {
		None
	}
}

impl ValueAsObject for NaiveTime {
	fn get_property(&self, _: &str) -> Option<&Value> {
		None
	}
}

impl ValueAsBool for NaiveTime {
	fn as_bool(&self) -> bool {
		true
	}
}
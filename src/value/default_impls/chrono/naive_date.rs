use super::chrono::NaiveDate;
use value::{Value, ValueAsString, ValueAsIterator, ValueAsObject, ValueAsBool};

impl ValueAsString for NaiveDate {
	fn as_string(&self) -> String {
		format!("{}", self)
	}
}

impl ValueAsIterator for NaiveDate {
	fn get_iterator(&self) -> Option<Box<Iterator<Item=&Value>>> {
		None
	}
}

impl ValueAsObject for NaiveDate {
	fn get_property(&self, _: &str) -> Option<&Value> {
		None
	}
}

impl ValueAsBool for NaiveDate {
	fn as_bool(&self) -> bool {
		true
	}
}
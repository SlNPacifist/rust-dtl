use std::fmt::Display;
use super::chrono::{DateTime, TimeZone};
use value::{Value, ValueAsString, ValueAsIterator, ValueAsObject, ValueAsBool};

impl<T: TimeZone> ValueAsString for DateTime<T> where T::Offset: Display {
	fn as_string(&self) -> String {
		format!("{}", self)
	}
}

impl<T: TimeZone> ValueAsIterator for DateTime<T> where T::Offset: Display {
	fn get_iterator(&self) -> Option<Box<Iterator<Item=&Value>>> {
		None
	}
}

impl<T: TimeZone> ValueAsObject for DateTime<T> where T::Offset: Display {
	fn get_property(&self, _: &str) -> Option<&Value> {
		None
	}
}

impl<T: TimeZone> ValueAsBool for DateTime<T> where T::Offset: Display {
	fn as_bool(&self) -> bool {
		true
	}
}
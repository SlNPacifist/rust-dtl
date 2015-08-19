use std::fmt::Display;
use super::chrono::{Date, TimeZone};
use value::{Value, ValueAsString, ValueAsIterator, ValueAsObject, ValueAsBool};

impl<T: TimeZone> ValueAsString for Date<T> where T::Offset: Display {
	fn as_string(&self) -> String {
		format!("{}", self)
	}
}

impl<T: TimeZone> ValueAsIterator for Date<T> where T::Offset: Display {
	fn get_iterator(&self) -> Option<Box<Iterator<Item=&Value>>> {
		None
	}
}

impl<T: TimeZone> ValueAsObject for Date<T> where T::Offset: Display {
	fn get_property(&self, _: &str) -> Option<&Value> {
		None
	}
}

impl<T: TimeZone> ValueAsBool for Date<T> where T::Offset: Display {
	fn as_bool(&self) -> bool {
		true
	}
}
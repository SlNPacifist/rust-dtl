extern crate chrono;
use value::Value;
use self::chrono::{NaiveDate, NaiveTime, NaiveDateTime, DateTime, Date, UTC, Local, FixedOffset};


pub fn apply(input: Option<Box<Value>>, arg: &str) -> Option<Box<Value>> {
	if input.is_none() {
		return None;
	}
	let content = input.unwrap();
	if let Some(dt) = content.downcast_ref::<DateTime<Local>>() {
		return Some(Box::new(dt.format(arg).to_string()));
	}
	if let Some(dt) = content.downcast_ref::<DateTime<FixedOffset>>() {
		return Some(Box::new(dt.format(arg).to_string()));
	}
	if let Some(dt) = content.downcast_ref::<DateTime<UTC>>() {
		return Some(Box::new(dt.format(arg).to_string()));
	}
	if let Some(dt) = content.downcast_ref::<NaiveDateTime>() {
		return Some(Box::new(dt.format(arg).to_string()));
	}
	if let Some(dt) = content.downcast_ref::<NaiveDate>() {
		return Some(Box::new(dt.format(arg).to_string()));
	}
	if let Some(dt) = content.downcast_ref::<NaiveTime>() {
		return Some(Box::new(dt.format(arg).to_string()));
	}
	if let Some(dt) = content.downcast_ref::<Date<Local>>() {
		return Some(Box::new(dt.format(arg).to_string()));
	}
	if let Some(dt) = content.downcast_ref::<Date<FixedOffset>>() {
		return Some(Box::new(dt.format(arg).to_string()));
	}
	if let Some(dt) = content.downcast_ref::<Date<UTC>>() {
		return Some(Box::new(dt.format(arg).to_string()));
	}
	None
}
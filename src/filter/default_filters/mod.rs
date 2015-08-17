mod default;
mod add;
use super::FilterFunction;

pub static DEFAULT_FILTERS: [(&'static str, FilterFunction); 2] = [
	("default", default::apply),
	("add", add::apply),
];
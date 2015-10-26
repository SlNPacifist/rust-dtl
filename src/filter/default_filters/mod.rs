mod default;
mod add;
mod not;
use super::FilterFunction;

pub static DEFAULT_FILTERS: [(&'static str, FilterFunction); 3] = [
	("default", default::apply),
	("add", add::apply),
	("not", not::apply),
];


#[cfg(feature = "chrono")]
mod chrono_format;

#[cfg(feature = "chrono")]
pub static CHRONO_FILTERS: [(&'static str, FilterFunction); 1] = [
	("chrono_format", chrono_format::apply),
];

#[cfg(not(feature = "chrono"))]
pub static CHRONO_FILTERS: [(&'static str, FilterFunction); 0] = [];

mod default_impls;
use std::fmt::Debug;

pub trait ValueAsStringByRef {
	fn as_string_ref(&self) -> &str;
}

pub trait ValueAsIterator {
	fn get_iterator<'a>(&'a self) -> Option<Box<Iterator<Item=&Value> + 'a>>;
}

pub trait ValueAsObject {
	fn get_property(&self, name: &str) -> Option<&Value>;
}

pub trait ValueClone {
    fn clone_box(&self) -> Box<Value>;
}

impl<T> ValueClone for T where T: 'static + Value + Clone {
    fn clone_box(&self) -> Box<Value> {
        Box::new(self.clone())
    }
}

impl Clone for Box<Value> {
    fn clone(&self) -> Box<Value> {
        self.clone_box()
    }
}

// We cannot use Clone trait here because it will cause trait to be not object-safe
pub trait Value: ValueAsStringByRef + ValueAsIterator + ValueAsObject + ValueClone + Debug {}
impl<T> Value for T where T: ValueAsStringByRef + ValueAsIterator + ValueAsObject + ValueClone + Debug {}
use value::Value;

pub fn value_to_trait_object<T: Value>(s: &T) -> &Value {
	s
}

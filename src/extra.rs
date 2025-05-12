use serde_json::Value;

pub trait Extra {
    fn insert_extra(&mut self, key: String, value: Value) -> Option<Value>;
    fn remove_extra(&mut self, key: &str) -> Option<Value>;
}

#[macro_export]
macro_rules! impl_extra {
    ($($t:ty),+ $(,)?) => ($(
        impl Extra for $t {
            /// Inserts an element into the Extra map.
            /// Returns the value that it replaced, if one was present, or None if not.
            fn insert_extra(&mut self, key: String, value: Value) -> Option<Value> {
                self.extra.insert(key, value)
            }

            /// Removes the value at the provided key.
            /// Returns the value if one was present, or None if not.
            fn remove_extra(&mut self, key: &str) -> Option<Value> {
                self.extra.remove(key)
            }
        }
    )+)
}

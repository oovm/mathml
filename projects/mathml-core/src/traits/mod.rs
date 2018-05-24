use std::{collections::BTreeMap, fmt::Display};

/// A trait for all MathML elements.
pub trait MathElement
where
    Self: Clone + Display,
{
    /// Modify all attributes directly
    fn mut_attributes(&mut self) -> &mut BTreeMap<String, String>;
    /// Add an attribute to the operator.
    fn add_attribute<K, V>(&mut self, key: K, value: V)
    where
        K: ToString,
        V: ToString,
    {
        self.mut_attributes().insert(key.to_string(), value.to_string());
    }
}

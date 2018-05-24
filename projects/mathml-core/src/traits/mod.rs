use std::{
    collections::BTreeMap,
    fmt::{Display, Formatter},
};

/// A trait for all MathML elements.
pub trait MathElement
where
    Self: Clone + Display,
{
    /// Get the tag name of the element.
    fn tag_name(&self) -> &'static str;
    /// Get all attributes directly
    fn get_attributes(&self) -> &BTreeMap<String, String>;
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
    /// Add an attribute to the operator.
    fn with_attribute<K, V>(mut self, key: K, value: V) -> Self
    where
        K: ToString,
        V: ToString,
    {
        self.add_attribute(key, value);
        self
    }
}

pub(crate) fn write_tag_start<T>(f: &mut Formatter<'_>, element: &T) -> std::fmt::Result
where
    T: MathElement,
{
    write!(f, "<{}", element.tag_name())?;
    for (key, value) in element.get_attributes() {
        write!(f, " {}=\"{}\"", key, value)?;
    }
    f.write_str(">")
}

pub(crate) fn write_tag_close<T>(f: &mut Formatter<'_>, element: &T) -> std::fmt::Result
where
    T: MathElement,
{
    write!(f, "</{}>", element.tag_name())
}

pub(crate) fn write_tag_self_close<T>(f: &mut Formatter<'_>, element: &T) -> std::fmt::Result
where
    T: MathElement,
{
    write!(f, "<{}", element.tag_name())?;
    for (key, value) in element.get_attributes() {
        write!(f, " {}=\"{}\"", key, value)?;
    }
    f.write_str("/>")
}

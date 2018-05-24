use std::{
    collections::BTreeMap,
    fmt::{Display, Formatter},
};

/// A trait for all MathML elements.
pub trait MathElement
where
    Self: Clone + Display,
{
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
    fn with_attribute<K, V>(mut self, key: K, value: V) -> Self
    where
        K: ToString,
        V: ToString,
    {
        self.add_attribute(key, value);
        self
    }
}

pub(crate) trait MathElementWriter
where
    Self: MathElement,
{
    fn write_tag_start(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}", self.tag_name())?;
        for (key, value) in self.get_attributes() {
            write!(f, " {}=\"{}\"", key, value)?;
        }
        f.write_str(">")
    }
    fn write_tag_end(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "</{}>", self.tag_name())
    }
}

pub(crate) fn write_tag_start<T>(element: &T, f: &mut Formatter<'_>) -> std::fmt::Result
where
    T: MathElement,
{
    write!(f, "<{}", element.tag_name())?;
    for (key, value) in element.get_attributes() {
        write!(f, " {}=\"{}\"", key, value)?;
    }
    f.write_str(">")
}

pub(crate) fn write_tag_end<T>(element: &T, f: &mut Formatter<'_>) -> std::fmt::Result
where
    T: MathElement,
{
    write!(f, "</{}>", element.tag_name())
}

use super::*;

impl Default for MathRoot {
    fn default() -> Self {
        Self { children: Vec::new(), attributes: BTreeMap::new() }
    }
}

impl MathRoot {
    pub fn new<I>(children: I) -> Self
    where
        I: IntoIterator<Item = MathML>,
    {
        Self { children: children.into_iter().collect(), ..Default::default() }
    }
    pub fn with_attribute<K, V>(mut self, key: K, value: V) -> MathRoot
    where
        K: ToString,
        V: ToString,
    {
        self.attributes.insert(key.to_string(), value.to_string());
        self
    }
    pub fn with_display_style(mut self) -> Self {
        self.with_attribute("display", "block")
    }
    pub fn with_inline_style(mut self) -> Self {
        self.with_attribute("display", "inline")
    }
}

impl MathPhantom {
    pub fn new(inner: MathML) -> Self {
        Self { inner }
    }
}

impl From<MathRoot> for MathML {
    fn from(value: MathRoot) -> Self {
        Self::Root(Box::new(value))
    }
}

impl From<MathPhantom> for MathML {
    fn from(value: MathPhantom) -> Self {
        Self::Phantom(Box::new(value))
    }
}

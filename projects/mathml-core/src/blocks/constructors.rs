use super::*;

impl Default for MathRoot {
    fn default() -> Self {
        Self { children: Vec::new(), attributes: BTreeMap::new() }
    }
}

impl MathRow {
    pub fn new<I>(items: I) -> Self
    where
        I: IntoIterator<Item = MathML>,
    {
        Self { children: items.into_iter().collect(), has_grouping: false }
    }
    pub fn group<I>(items: I) -> Self
    where
        I: IntoIterator<Item = MathML>,
    {
        Self { children: items.into_iter().collect(), has_grouping: true }
    }
    pub fn get_items(&self) -> &[MathML] {
        &self.children
    }
    pub fn mut_items(&mut self) -> &mut Vec<MathML> {
        &mut self.children
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
    pub fn with_display_style(self) -> Self {
        self.with_attribute("display", "block")
    }
    pub fn with_inline_style(self) -> Self {
        self.with_attribute("display", "inline")
    }
    pub fn with_namespace(self) -> Self {
        self.with_attribute("xmlns", "http://www.w3.org/1998/Math/MathML")
    }
}

impl MathPhantom {
    pub fn new(inner: MathML) -> Self {
        Self { inner }
    }
    pub fn get_inner(&self) -> &MathML {
        &self.inner
    }
    pub fn mut_inner(&mut self) -> &mut MathML {
        &mut self.inner
    }
}

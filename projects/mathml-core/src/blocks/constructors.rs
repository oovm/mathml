use super::*;

impl Default for MathRoot {
    fn default() -> Self {
        Self { children: Vec::new(), attributes: BTreeMap::new() }
    }
}

impl MathElement for MathRoot {
    fn mut_attributes(&mut self) -> &mut BTreeMap<String, String> {
        &mut self.attributes
    }
}

impl MathRoot {
    pub fn new<I>(children: I) -> Self
    where
        I: IntoIterator<Item = MathML>,
    {
        Self { children: children.into_iter().collect(), ..Default::default() }
    }
    pub fn with_display_style(self, display: bool) -> Self {
        let display = if display { "block" } else { "inline" };
        self.with_attribute("display", display)
    }
    pub fn with_namespace(self) -> Self {
        self.with_attribute("xmlns", "http://www.w3.org/1998/Math/MathML")
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

impl MathElement for MathStyle {
    fn mut_attributes(&mut self) -> &mut BTreeMap<String, String> {
        &mut self.attributes
    }
}

// noinspection DuplicatedCode
impl MathStyle {
    pub fn display<M>(base: M) -> Self
    where
        M: Into<MathML>,
    {
        Self { base: base.into(), attributes: Default::default() }.with_attribute("displaystyle", "true")
    }
    pub fn inline<M>(base: M) -> Self
    where
        M: Into<MathML>,
    {
        Self { base: base.into(), attributes: Default::default() }.with_attribute("displaystyle", "false")
    }
    /// Add an attribute to the operator.
    pub fn add_attribute<K, V>(&mut self, key: K, value: V)
    where
        K: ToString,
        V: ToString,
    {
        self.attributes.insert(key.to_string(), value.to_string());
    }
    /// Modify all attributes directly
    pub fn mut_attributes(&mut self) -> &mut BTreeMap<String, String> {
        &mut self.attributes
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

impl MathFunction {
    pub fn new<S, I>(name: S, body: I) -> Self
    where
        S: ToString,
        I: IntoIterator<Item = MathML>,
    {
        Self { name: name.to_string(), body: body.into_iter().collect() }
    }
    pub fn add_argument(&mut self, argument: MathML) {
        self.body.push(argument);
    }
    pub fn get_arguments(&self) -> &[MathML] {
        &self.body
    }
    pub fn mut_arguments(&mut self) -> &mut Vec<MathML> {
        &mut self.body
    }
}

// noinspection DuplicatedCode
impl MathTable {
    pub fn matrix<I>(stream: I) -> Self
    where
        I: IntoIterator<Item = MathML>,
    {
        Self { stream: stream.into_iter().collect(), attributes: BTreeMap::new() }
    }
    /// Add an attribute to the operator.
    pub fn add_attribute<K, V>(&mut self, key: K, value: V)
    where
        K: ToString,
        V: ToString,
    {
        self.attributes.insert(key.to_string(), value.to_string());
    }
    /// Modify all attributes directly
    pub fn mut_attributes(&mut self) -> &mut BTreeMap<String, String> {
        &mut self.attributes
    }
}

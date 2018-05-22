use super::*;

impl Default for MathSpace {
    fn default() -> Self {
        MathSpace::new(1.0)
    }
}

impl MathOperator {
    pub fn new<S>(text: S) -> Self
    where
        S: ToString,
    {
        Self { operator: text.to_string(), attributes: Default::default() }
    }
    pub fn get_attribute(&self, key: &str) -> Option<&str> {
        self.attributes.get(key).map(|s| s.as_str())
    }
    pub fn add_attribute<K, V>(&mut self, key: K, value: V)
    where
        K: ToString,
        V: ToString,
    {
        self.attributes.insert(key.to_string(), value.to_string());
    }
    pub fn with_attribute<K, V>(mut self, key: K, value: V) -> Self
    where
        K: ToString,
        V: ToString,
    {
        self.add_attribute(key, value);
        self
    }
    pub fn mut_attributes(&mut self) -> &mut BTreeMap<String, String> {
        &mut self.attributes
    }
}

impl MathSpace {
    pub fn new(width: f32) -> Self {
        let mut attributes = BTreeMap::new();
        attributes.insert("width".to_string(), format!("{}rem", width));
        Self { attributes }
    }
    pub fn get_attribute(&self, key: &str) -> Option<&str> {
        self.attributes.get(key).map(|s| s.as_str())
    }
    pub fn add_attribute<K, V>(&mut self, key: K, value: V)
    where
        K: ToString,
        V: ToString,
    {
        self.attributes.insert(key.to_string(), value.to_string());
    }
    pub fn with_attribute<K, V>(mut self, key: K, value: V) -> Self
    where
        K: ToString,
        V: ToString,
    {
        self.add_attribute(key, value);
        self
    }
    pub fn mut_attributes(&mut self) -> &mut BTreeMap<String, String> {
        &mut self.attributes
    }
}

impl MathSqrt {
    pub fn new(base: MathML) -> Self {
        Self { base, surd: None }
    }
    pub fn surd(base: MathML, power: MathML) -> Self {
        Self { base, surd: Some(power) }
    }
}

impl MathMultiScript {
    pub fn new(base: MathML, lu: Vec<MathML>, ld: Vec<MathML>, ru: Vec<MathML>, rd: Vec<MathML>) -> Self {
        Self { base, ru, rd, lu, ld, attributes: BTreeMap::new() }
    }
    pub fn sub_script(base: MathML, sub: MathML) -> Self {
        MathMultiScript::new(base, vec![], vec![], vec![], vec![sub])
    }
    pub fn is_sub_script(&self) -> bool {
        self.lu.is_empty() && self.ld.is_empty() && self.ru.is_empty() && self.rd.len() == 1
    }
    pub fn super_script(base: MathML, sup: MathML) -> Self {
        MathMultiScript::new(base, vec![], vec![], vec![sup], vec![])
    }
    pub fn is_super_script(&self) -> bool {
        self.lu.is_empty() && self.ld.is_empty() && self.ru.len() == 1 && self.rd.is_empty()
    }
    pub fn sub_super_script(base: MathML, sub: MathML, sup: MathML) -> Self {
        MathMultiScript::new(base, vec![], vec![], vec![sup], vec![sub])
    }
    pub fn is_sub_super_script(&self) -> bool {
        self.lu.is_empty() && self.ld.is_empty() && self.ru.len() == 1 && self.rd.len() == 1
    }
}

impl MathFenced {
    pub fn new<I>(base: I, lhs: char, rhs: char) -> Self
    where
        I: IntoIterator<Item = MathML>,
    {
        Self { base: base.into_iter().collect(), open: lhs, close: rhs, separators: String::new() }
    }
    pub fn parentheses<I>(base: I) -> Self
    where
        I: IntoIterator<Item = MathML>,
    {
        Self::new(base, '(', ')')
    }

    pub fn brackets<I>(base: I) -> Self
    where
        I: IntoIterator<Item = MathML>,
    {
        Self::new(base, '[', ']')
    }
    pub fn curly<I>(base: I) -> Self
    where
        I: IntoIterator<Item = MathML>,
    {
        Self::new(base, '{', '}')
    }
    pub fn with_separators<S>(mut self, separators: S) -> Self
    where
        S: ToString,
    {
        self.separators = separators.to_string();
        self
    }
}

impl MathUnderOver {
    pub fn under(base: MathML, under: MathML) -> Self {
        Self { base, under: Some(under), over: None, attributes: BTreeMap::new() }
    }
    pub fn over(base: MathML, over: MathML) -> Self {
        Self { base, under: None, over: Some(over), attributes: BTreeMap::new() }
    }
    pub fn under_over(base: MathML, under: MathML, over: MathML) -> Self {
        Self { base, under: Some(under), over: Some(over), attributes: BTreeMap::new() }
    }
    pub fn with_accent_over(mut self, accent: bool) -> Self {
        if accent {
            self.attributes.insert("accent".to_string(), "true".to_string());
        }
        else {
            self.attributes.remove("accent");
        }
        self
    }
    pub fn with_accent_under(mut self, accent: bool) -> Self {
        if accent {
            self.attributes.insert("accentunder".to_string(), "true".to_string());
        }
        else {
            self.attributes.remove("accentunder");
        }
        self
    }
}
impl MathML {
    pub fn operation<S>(text: S) -> Self
    where
        S: ToString,
    {
        MathOperator::new(text).into()
    }
}

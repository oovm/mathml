use super::*;

impl MathOperator {
    pub fn new<S>(text: S) -> Self
    where
        S: ToString,
    {
        Self { operator: text.to_string() }
    }
}

impl Default for MathSpace {
    fn default() -> Self {
        MathSpace::new(1.0)
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
    pub fn add_attribute<K, V>(mut self, key: K, value: V) -> Self
    where
        K: ToString,
        V: ToString,
    {
        self.attributes.insert(key.to_string(), value.to_string());
        self
    }
    pub fn remove_attribute(&mut self, key: &str) -> Option<String> {
        self.attributes.remove(key)
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
    pub fn new(base: MathML, ru: Vec<MathML>, rd: Vec<MathML>, lu: Vec<MathML>, ld: Vec<MathML>) -> Self {
        Self { base, ru, rd, lu, ld, attributes: BTreeMap::new() }
    }
    pub fn sub_script(base: MathML, sub: MathML) -> Self {
        Self { base, ru: Vec::new(), rd: Vec::new(), lu: Vec::new(), ld: vec![sub], attributes: BTreeMap::new() }
    }
    pub fn is_sub_script(&self) -> bool {
        self.ru.is_empty() && self.rd.is_empty() && self.lu.is_empty() && self.ld.len() == 1
    }
    pub fn super_script(base: MathML, sup: MathML) -> Self {
        Self { base, ru: Vec::new(), rd: Vec::new(), lu: vec![sup], ld: Vec::new(), attributes: BTreeMap::new() }
    }
    pub fn is_super_script(&self) -> bool {
        self.ru.is_empty() && self.rd.is_empty() && self.lu.len() == 1 && self.ld.is_empty()
    }
    pub fn sub_super_script(base: MathML, sub: MathML, sup: MathML) -> Self {
        Self { base, ru: Vec::new(), rd: Vec::new(), lu: vec![sup], ld: vec![sub], attributes: BTreeMap::new() }
    }
    pub fn is_sub_super_script(&self) -> bool {
        self.ru.is_empty() && self.rd.is_empty() && self.lu.len() == 1 && self.ld.len() == 1
    }
}

impl MathFenced {
    pub fn new<I>(base: I, lhs: char, rhs: char) -> Self
    where
        I: IntoIterator<Item = MathML>,
    {
        Self { base: Vec::from_iter(base.into_iter()), open: lhs, close: rhs, separators: String::new() }
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

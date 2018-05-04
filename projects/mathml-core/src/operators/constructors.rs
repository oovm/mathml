use super::*;

impl MathOperator {
    pub fn new<S>(text: S) -> Self
    where
        S: ToString,
    {
        Self { operator: text.to_string() }
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
    pub fn new<I, S>(base: I, lhs: S, rhs: S) -> Self
    where
        I: IntoIterator<Item = MathML>,
        S: Into<String>,
    {
        Self { base: Vec::from_iter(base.into_iter()), open: lhs.into(), close: rhs.into(), separators: String::new() }
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

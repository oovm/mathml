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

impl MathSub {
    pub fn new(base: MathML, sub: MathML) -> Self {
        Self { base, sub }
    }
}

impl MathSup {
    pub fn new(base: MathML, sup: MathML) -> Self {
        Self { base, sup }
    }
}

impl MathSubSup {
    pub fn new(base: MathML, sub: MathML, sup: MathML) -> Self {
        Self { base, sub, sup }
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

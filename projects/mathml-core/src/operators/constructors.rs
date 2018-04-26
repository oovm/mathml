use super::*;

impl MathOperator {
    pub fn new<S>(text: S) -> Self
    where
        S: ToString,
    {
        Self { operator: text.to_string() }
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

impl MathML {
    pub fn operation<S>(text: S) -> Self
    where
        S: ToString,
    {
        MathOperator::new(text).into()
    }
}

impl From<MathOperator> for MathML {
    fn from(value: MathOperator) -> Self {
        MathML::Operator(Box::new(value))
    }
}

impl From<MathSub> for MathML {
    fn from(value: MathSub) -> Self {
        MathML::Sub(Box::new(value))
    }
}

impl From<MathSup> for MathML {
    fn from(value: MathSup) -> Self {
        MathML::Sup(Box::new(value))
    }
}

impl From<MathSubSup> for MathML {
    fn from(value: MathSubSup) -> Self {
        MathML::SubSup(Box::new(value))
    }
}

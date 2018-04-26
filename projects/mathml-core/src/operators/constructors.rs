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

impl MathML {
    pub fn operation<S>(text: S) -> Self
    where
        S: ToString,
    {
        MathOperator::new(text).into()
    }
}

use super::*;

// noinspection SpellCheckingInspection
impl MathElement for MathOperator {
    fn tag_name(&self) -> &'static str {
        "mo"
    }

    fn get_attributes(&self) -> &BTreeMap<String, String> {
        &self.attributes
    }

    fn mut_attributes(&mut self) -> &mut BTreeMap<String, String> {
        &mut self.attributes
    }
}

// noinspection SpellCheckingInspection
impl MathOperator {
    /// Create a simple math operator without any attributes.
    pub fn new<S>(text: S) -> Self
    where
        S: ToString,
    {
        Self { operator: text.to_string(), attributes: Default::default() }
    }
    /// Mark the operator as a fence (such as parentheses). There is no visual effect for this attribute.
    pub fn mark_fence(self) -> Self {
        self.with_attribute("fence", true)
    }
    ///  Mark the operator as a separator (such as commas). There is no visual effect for this attribute.
    pub fn mark_separator(self) -> Self {
        self.with_attribute("separator", true)
    }
    /// Mark the operator should be drawn bigger when math-style is set to normal.
    pub fn mark_large_operator(self) -> Self {
        self.with_attribute("largeop", true)
    }
    /// Mark the operator stretches to the size of the adjacent element.
    pub fn mark_stretchy(self) -> Self {
        self.with_attribute("stretchy", true)
    }
    /// Mark the stretchy operator should be vertically symmetric around the imaginary math axis (centered fraction line).
    pub fn mark_symmetric(self) -> Self {
        self.mark_stretchy().with_attribute("symmetric", true)
    }
    /// A <length-percentage> indicating the amount of space before the operator.
    /// A <length-percentage> indicating the amount of space after the operator.
    pub fn with_space(mut self, lhs: f32, rhs: f32) -> Self {
        self.with_attribute("lspace", lhs).with_attribute("rspace", rhs)
    }
    /// A <length-percentage> indicating the maximum size of the operator when it is stretchy.
    pub fn with_size(mut self, min: f32, max: f32) -> Self {
        self.mark_stretchy().with_attribute("minsize", min).with_attribute("maxsize", max)
    }
}

impl Default for MathSpace {
    fn default() -> Self {
        MathSpace::new(1.0)
    }
}

// noinspection SpellCheckingInspection
impl MathElement for MathSpace {
    fn tag_name(&self) -> &'static str {
        "mspace"
    }

    fn get_attributes(&self) -> &BTreeMap<String, String> {
        &self.attributes
    }

    fn mut_attributes(&mut self) -> &mut BTreeMap<String, String> {
        &mut self.attributes
    }
}

impl MathSpace {
    /// Create a simple math space without any attributes, the unit is `rem`.
    pub fn new(width: f32) -> Self {
        let mut attributes = BTreeMap::new();
        attributes.insert("width".to_string(), format!("{}rem", width));
        Self { attributes }
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

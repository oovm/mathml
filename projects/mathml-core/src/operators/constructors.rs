use super::*;

// noinspection DuplicatedCode
// noinspection SpellCheckingInspection
impl MathOperator {
    /// Create a simple math operator without any attributes.
    pub fn new<S>(text: S) -> Self
    where
        S: ToString,
    {
        Self { operator: text.to_string(), attributes: Default::default() }
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
    /// Mark the operator as a fence (such as parentheses). There is no visual effect for this attribute.
    pub fn mark_fence(mut self) -> Self {
        self.add_attribute("fence", true);
        self
    }
    ///  Mark the operator as a separator (such as commas). There is no visual effect for this attribute.
    pub fn mark_separator(mut self) -> Self {
        self.add_attribute("separator", true);
        self
    }
    /// Mark the operator should be drawn bigger when math-style is set to normal.
    pub fn mark_large_operator(mut self) -> Self {
        self.add_attribute("largeop", true);
        self
    }
    /// Mark the operator stretches to the size of the adjacent element.
    pub fn mark_stretchy(mut self) -> Self {
        self.add_attribute("stretchy", true);
        self
    }
    /// Mark the stretchy operator should be vertically symmetric around the imaginary math axis (centered fraction line).
    pub fn mark_symmetric(mut self) -> Self {
        self.add_attribute("symmetric", true);
        self.mark_stretchy()
    }
    /// A <length-percentage> indicating the amount of space before the operator.
    /// A <length-percentage> indicating the amount of space after the operator.
    pub fn with_space(mut self, lhs: f32, rhs: f32) -> Self {
        self.add_attribute("lspace", lhs);
        self.add_attribute("rspace", rhs);
        self
    }
    /// A <length-percentage> indicating the maximum size of the operator when it is stretchy.
    pub fn with_size(mut self, min: f32, max: f32) -> Self {
        self.add_attribute("minsize", min);
        self.add_attribute("maxsize", max);
        self.mark_stretchy()
    }
}

impl Default for MathSpace {
    fn default() -> Self {
        MathSpace::new(1.0)
    }
}

// noinspection DuplicatedCode
impl MathSpace {
    /// Create a simple math space without any attributes, the unit is `rem`.
    pub fn new(width: f32) -> Self {
        let mut attributes = BTreeMap::new();
        attributes.insert("width".to_string(), format!("{}rem", width));
        Self { attributes }
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

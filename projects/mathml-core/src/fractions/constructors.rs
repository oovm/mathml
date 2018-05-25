use super::*;

impl MathFraction {
    /// Creates a new [`MathFraction`].
    pub fn new<N, D>(numerator: N, denominator: D) -> Self
    where
        N: Into<MathML>,
        D: Into<MathML>,
    {
        Self { numerator: numerator.into(), denominator: denominator.into(), line_thickness: Default::default() }
    }
    /// Creates a new [`MathFraction`] with the given [`LineThickness`].
    pub fn with_thickness<T>(mut self, line_thickness: T) -> Self
    where
        T: Into<LineThickness>,
    {
        self.line_thickness = line_thickness.into();
        self
    }
}

impl Default for LineThickness {
    fn default() -> Self {
        LineThickness::Medium
    }
}

// noinspection SpellCheckingInspection
impl MathElement for MathFraction {
    fn tag_name(&self) -> &'static str {
        "mfrac"
    }

    fn get_attributes(&self) -> &BTreeMap<String, String> {
        todo!()
    }

    fn mut_attributes(&mut self) -> &mut BTreeMap<String, String> {
        todo!()
    }
}

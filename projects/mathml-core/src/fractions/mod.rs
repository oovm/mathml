mod display;

impl MathFraction {
    pub fn new(numerator: MathML, denominator: MathML) -> Self {
        Self { numerator, denominator, line_thickness: None }
    }
    pub fn with_thickness(mut self, line_thickness: f32) -> Self {
        self.line_thickness = Some(line_thickness);
        self
    }
}
impl From<MathFraction> for MathML {
    fn from(value: MathFraction) -> Self {
        MathML::Frac(Box::new(value))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineThickness {
    Thin,
    Medium,
    Thick,
    Length(u8),
}

impl Display for LineThickness {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LineThickness::Thin => write!(f, r#" linethickness="thin""#),
            LineThickness::Medium => write!(f, r#""#),
            LineThickness::Thick => write!(f, r#" linethickness="medium""#),
            LineThickness::Length(l) => write!(f, r#" linethickness="{}""#, l),
        }
    }
}

impl MathML {}

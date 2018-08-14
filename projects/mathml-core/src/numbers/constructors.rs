use super::*;

impl MathError {
    /// Creates a new [`MathError`].
    pub fn new<S>(message: S) -> Self
    where
        S: ToString,
    {
        Self { message: message.to_string() }
    }
}

impl MathNumber {
    /// Creates a new [`MathNumber`] with the given number.
    pub fn new<S>(text: S) -> Self
    where
        S: ToString,
    {
        Self { number: text.to_string() }
    }
}

macro_rules! make_number {
    ($($t:ty),*) => {
        $(
            impl From<$t> for MathNumber {
                fn from(value: $t) -> Self {
                    Self::new(value)
                }
            }
        )*
    };
}

make_number!(i8, i16, i32, i64, i128, isize);
make_number!(u8, u16, u32, u64, u128, usize);
make_number!(f32, f64);

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

impl MathML {
    /// Creates a new [`MathNumber`] with the given number.
    pub fn number<N>(n: N) -> Self
    where
        N: Into<MathNumber>,
    {
        n.into().into()
    }
    /// Creates a new [`MathError`] with the given message.
    pub fn error<S>(message: S) -> Self
    where
        S: ToString,
    {
        MathError::new(message).into()
    }
}

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

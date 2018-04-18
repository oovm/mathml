use std::fmt::{Display, Formatter};
mod display;
#[derive(Debug, Clone, PartialEq)]
pub struct MathNumber {
    number: String,
}

impl MathNumber {}

macro_rules! make_number {
    ($($t:ty),*) => {
        $(
            impl From<$t> for MathNumber {
                fn from(value: $t) -> Self {
                    Self { number: value.to_string() }
                }
            }
        )*
    };
}

make_number!(i8, i16, i32, i64, i128, isize);
make_number!(u8, u16, u32, u64, u128, usize);
make_number!(f32, f64);

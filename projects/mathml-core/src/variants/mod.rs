use crate::MathVariant;
use std::fmt::{Display, Formatter};
mod display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MathLetter {
    pub letter: char,
    pub variant: MathVariant,
}

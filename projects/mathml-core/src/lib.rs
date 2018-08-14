#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

mod ast;
mod blocks;
mod identifiers;
mod numbers;
mod operators;
mod traits;

pub mod helpers;

pub use crate::{
    ast::MathML,
    blocks::{MathFunction, MathPhantom, MathRoot, MathRow, MathStyle, MathTable},
    identifiers::{FontVariant, MathIdentifier, MathText},
    numbers::{LineThickness, MathError, MathFraction, MathNumber},
    operators::{MathFenced, MathMultiScript, MathOperator, MathSpace, MathSqrt, MathUnderOver},
    traits::MathElement,
};

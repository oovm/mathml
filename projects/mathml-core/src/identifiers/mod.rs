use crate::{helpers::safe_html_str, MathML};
use std::fmt::{Display, Formatter};

mod constructors;
mod display;

/// The [`<mi>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mi) element indicates that the content should be rendered as an identifier such as function names, variables or symbolic constants.
///
/// You can also have arbitrary text in it to mark up terms.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MathIdentifier {
    identifier: String,
    variant: FontVariant,
}

// noinspection SpellCheckingInspection
/// The [`<mtext>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mtext) element is used to display text in a math formula.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MathText {
    is_string: bool,
    text: String,
}

// noinspection SpellCheckingInspection
/// Font variant for [`<mi>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mi),
/// used as [`mathvariant`](https://developer.mozilla.org/en-US/docs/Web/MathML/Global_attributes/mathvariant) attribute.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FontVariant {
    /// <math><mi mathvariant="normal">ABCDEFGHIJKLMNOPQRSTUVWXYZ</mi></math>
    Normal,
    /// <math><mi mathvariant="italic">ABCDEFGHIJKLMNOPQRSTUVWXYZ</mi></math>
    Italic,
    /// <math><mi mathvariant="bold">ABCDEFGHIJKLMNOPQRSTUVWXYZ</mi></math>
    Bold,
    /// <math><mi mathvariant="bold-italic">ABCDEFGHIJKLMNOPQRSTUVWXYZ</mi></math>
    BoldItalic,
    /// <math><mi mathvariant="double-struck">ABCDEFGHIJKLMNOPQRSTUVWXYZ</mi></math>
    DoubleStruck,
    /// <math><mi mathvariant="bold-fraktur">ABCDEFGHIJKLMNOPQRSTUVWXYZ</mi></math>
    BoldFraktur,
    /// <math><mi mathvariant="script">ABCDEFGHIJKLMNOPQRSTUVWXYZ</mi></math>
    Script,
    /// <math><mi mathvariant="bold-script">ABCDEFGHIJKLMNOPQRSTUVWXYZ</mi></math>
    BoldScript,
    /// <math><mi mathvariant="fraktur">ABCDEFGHIJKLMNOPQRSTUVWXYZ</mi></math>
    Fraktur,
    /// <math><mi mathvariant="sans-serif">ABCDEFGHIJKLMNOPQRSTUVWXYZ</mi></math>
    SansSerif,
    /// <math><mi mathvariant="bold-sans-serif">ABCDEFGHIJKLMNOPQRSTUVWXYZ</mi></math>
    BoldSansSerif,
    /// <math><mi mathvariant="sans-serif-italic">ABCDEFGHIJKLMNOPQRSTUVWXYZ</mi></math>
    SansSerifItalic,
    /// <math><mi mathvariant="sans-serif-bold-italic">ABCDEFGHIJKLMNOPQRSTUVWXYZ</mi></math>
    SansSerifBoldItalic,
    /// <math><mi mathvariant="monospace">ABCDEFGHIJKLMNOPQRSTUVWXYZ</mi></math>
    Monospace,
}

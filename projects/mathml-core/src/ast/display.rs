use super::*;

impl Display for MathML {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MathML::Root(v) => Display::fmt(v, f),
            MathML::Row(v) => Display::fmt(v, f),
            MathML::Function(v) => Display::fmt(v, f),
            MathML::Number(v) => Display::fmt(v, f),
            MathML::Identifier(v) => Display::fmt(v, f),
            MathML::Operator(v) => Display::fmt(v, f),
            MathML::MultiScripts(v) => Display::fmt(v, f),
            MathML::UnderOver(v) => Display::fmt(v, f),
            MathML::Sqrt(v) => Display::fmt(v, f),
            MathML::Frac(v) => Display::fmt(v, f),
            MathML::Phantom(v) => Display::fmt(v, f),
            MathML::Fenced(v) => Display::fmt(v, f),
            MathML::Table(v) => Display::fmt(v, f),
            MathML::Space(v) => Display::fmt(v, f),
            MathML::Text(v) => Display::fmt(v, f),
            MathML::Style(v) => Display::fmt(v, f),
            MathML::Ampersand => Ok(()),
            MathML::NewLine => Ok(()),
            MathML::Undefined(_) => {
                todo!()
            }
            MathML::Nothing => Ok(()),
        }
    }
}

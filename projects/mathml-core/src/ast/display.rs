use super::*;

impl Display for DisplayStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplayStyle::Block => write!(f, "block"),
            DisplayStyle::Inline => write!(f, "inline"),
        }
    }
}

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
            MathML::OverOp(op, acc, target) => write!(f, r#"<mover>{}<mo accent="{}">{}</mo></mover>"#, target, acc, op),
            MathML::UnderOp(op, acc, target) => write!(f, r#"<munder>{}<mo accent="{}">{}</mo></munder>"#, target, acc, op),
            MathML::UnderOver(v) => Display::fmt(v, f),
            MathML::Sqrt(v) => Display::fmt(v, f),
            MathML::Frac(v) => Display::fmt(v, f),
            MathML::Phantom(v) => Display::fmt(v, f),
            MathML::Fenced(v) => Display::fmt(v, f),
            MathML::StrechedOp(stretchy, op) => write!(f, r#"<mo stretchy="{}">{}</mo>"#, stretchy, op),
            MathML::SizedParen { size, paren } => {
                write!(f, r#"<mrow><mo maxsize="{0}" minsize="{0}">{1}</mro></mrow>"#, size, paren)
            }
            MathML::Slashed(node) => match &**node {
                // force set math-variant here
                MathML::Identifier(mi) => {
                    write!(f, "{:}", mi)
                }
                MathML::Operator(x) => write!(f, "<mo>{}&#x0338;</mo>", x),
                n => write!(f, "{}", n),
            },
            MathML::Table(v) => Display::fmt(v, f),
            MathML::Space(v) => Display::fmt(v, f),
            MathML::Text(v) => Display::fmt(v, f),
            MathML::Style(display, content) => match display {
                Some(DisplayStyle::Block) => write!(f, r#"<mstyle displaystyle="true">{}</mstyle>"#, content),
                Some(DisplayStyle::Inline) => write!(f, r#"<mstyle displaystyle="false">{}</mstyle>"#, content),
                None => write!(f, "<mstyle>{}</mstyle>", content),
            },
            MathML::Ampersand => Ok(()),
            MathML::NewLine => Ok(()),
            MathML::Undefined(_) => {
                todo!()
            }
        }
    }
}

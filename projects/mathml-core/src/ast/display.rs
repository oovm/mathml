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
            MathML::Number(v) => Display::fmt(v, f),
            MathML::Identifier(v) => Display::fmt(v, f),
            MathML::Operator(v) => Display::fmt(v, f),
            MathML::Function(fun, arg) => match arg {
                Some(arg) => write!(f, "<mi>{}</mi><mo>&#x2061;</mo>{}", fun, arg),
                None => write!(f, "<mi>{}</mi>", fun),
            },
            MathML::Space(space) => write!(f, r#"<mspace width="{}em"/>"#, space),
            MathML::MultiScripts(v) => Display::fmt(v, f),
            MathML::OverOp(op, acc, target) => write!(f, r#"<mover>{}<mo accent="{}">{}</mo></mover>"#, target, acc, op),
            MathML::UnderOp(op, acc, target) => write!(f, r#"<munder>{}<mo accent="{}">{}</mo></munder>"#, target, acc, op),
            MathML::UnderOver(v) => Display::fmt(v, f),
            MathML::Sqrt(v) => Display::fmt(v, f),
            MathML::Frac(v) => Display::fmt(v, f),
            MathML::Row(vec) => write!(f, "<mrow>{}</mrow>", vec.iter().map(|node| format!("{}", node)).collect::<String>()),
            MathML::Phantom(v) => Display::fmt(v, f),
            MathML::Fenced { open, close, content } => {
                write!(
                    f,
                    r#"<mrow><mo stretchy="true" form="prefix">{}</mo>{}<mo stretchy="true" form="postfix">{}</mo></mrow>"#,
                    open, content, close
                )
            }
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
            MathML::Matrix(content, columnalign) => {
                let mut mathml = format!("<mtable{}><mtr><mtd>", columnalign);
                for (i, node) in content.iter().enumerate() {
                    match node {
                        MathML::NewLine => {
                            mathml.push_str("</mtd></mtr>");
                            if i < content.len() {
                                mathml.push_str("<mtr><mtd>")
                            }
                        }
                        MathML::Ampersand => {
                            mathml.push_str("</mtd>");
                            if i < content.len() {
                                mathml.push_str("<mtd>")
                            }
                        }
                        node => {
                            mathml = format!("{}{}", mathml, node);
                        }
                    }
                }
                mathml.push_str("</mtd></mtr></mtable>");

                write!(f, "{}", mathml)
            }
            MathML::Text(text) => write!(f, "<mtext>{}</mtext>", text),
            MathML::Style(display, content) => match display {
                Some(DisplayStyle::Block) => write!(f, r#"<mstyle displaystyle="true">{}</mstyle>"#, content),
                Some(DisplayStyle::Inline) => write!(f, r#"<mstyle displaystyle="false">{}</mstyle>"#, content),
                None => write!(f, "<mstyle>{}</mstyle>", content),
            },
            MathML::Ampersand => {
                todo!()
            }
            MathML::NewLine => {
                todo!()
            }
            MathML::Undefined(_) => {
                todo!()
            }
        }
    }
}

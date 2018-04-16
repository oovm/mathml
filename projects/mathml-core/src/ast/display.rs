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
            MathML::Number(number) => write!(f, "<mn>{}</mn>", number),
            MathML::Letter(v) => Display::fmt(v, f),
            MathML::Operator(op) => {
                if op == &'∂' {
                    write!(f, r#"<mo mathvariant="italic">∂</mo>"#)
                }
                else {
                    write!(f, r#"<mo>{}</mo>"#, op)
                }
            }
            MathML::Function(fun, arg) => match arg {
                Some(arg) => write!(f, "<mi>{}</mi><mo>&#x2061;</mo>{}", fun, arg),
                None => write!(f, "<mi>{}</mi>", fun),
            },
            MathML::Space(space) => write!(f, r#"<mspace width="{}em"/>"#, space),
            MathML::Subscript(a, b) => write!(f, "<msub>{}{}</msub>", a, b),
            MathML::Superscript(a, b) => write!(f, "<msup>{}{}</msup>", a, b),
            MathML::SubSup { target, sub, sup } => write!(f, "<msubsup>{}{}{}</msubsup>", target, sub, sup),
            MathML::OverOp(op, acc, target) => write!(f, r#"<mover>{}<mo accent="{}">{}</mo></mover>"#, target, acc, op),
            MathML::UnderOp(op, acc, target) => write!(f, r#"<munder>{}<mo accent="{}">{}</mo></munder>"#, target, acc, op),
            MathML::Overset { over, target } => write!(f, r#"<mover>{}{}</mover>"#, target, over),
            MathML::Underset { under, target } => write!(f, r#"<munder>{}{}</munder>"#, target, under),
            MathML::Under(target, under) => write!(f, r#"<munder>{}{}</munder>"#, target, under),
            MathML::UnderOver { target, under, over } => write!(f, r#"<munderover>{}{}{}</munderover>"#, target, under, over),
            MathML::Sqrt(degree, content) => match degree {
                Some(deg) => write!(f, "<mroot>{}{}</mroot>", content, deg),
                None => write!(f, "<msqrt>{}</msqrt>", content),
            },
            MathML::Frac(num, denom, lt) => write!(f, "<mfrac{}>{}{}</mfrac>", lt, num, denom),
            MathML::Row(vec) => write!(f, "<mrow>{}</mrow>", vec.iter().map(|node| format!("{}", node)).collect::<String>()),
            MathML::Fenced { open, close, content } => {
                write!(
                    f,
                    r#"<mrow><mo stretchy="true" form="prefix">{}</mo>{}<mo stretchy="true" form="postfix">{}</mo></mrow>"#,
                    open, content, close
                )
            }
            MathML::StrechedOp(stretchy, op) => write!(f, r#"<mo stretchy="{}">{}</mo>"#, stretchy, op),
            MathML::OtherOperator(op) => write!(f, "<mo>{}</mo>", op),
            MathML::SizedParen { size, paren } => {
                write!(f, r#"<mrow><mo maxsize="{0}" minsize="{0}">{1}</mro></mrow>"#, size, paren)
            }
            MathML::Slashed(node) => match &**node {
                MathML::Letter(MathIdentifier { letter, variant }) => {
                    write!(f, "<mi mathvariant=\"{}\">{}&#x0338;</mi>", variant, letter)
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
            node => write!(f, "<mtext>[PARSE ERROR: {:?}]</mtext>", node),
        }
    }
}

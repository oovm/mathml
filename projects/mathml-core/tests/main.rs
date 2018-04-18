#[test]
fn ready() {
    println!("it works!")
}

use mathml_core::{MathIdentifier, MathML, MathNumber};

#[test]
fn node_display() {
    let problems = vec![
        (MathML::Number(Box::new(MathNumber::from(3.14))), "<mn>3.14</mn>"),
        (MathML::Letter(Box::new(MathIdentifier::italic('a'))), "<mi>a</mi>"),
        (MathML::Letter(Box::new(MathIdentifier::italic('α'))), "<mi>α</mi>"),
        (MathML::Letter(Box::new(MathIdentifier::italic('&'))), "<mi>&amp;</mi>"),
        (MathML::Letter(Box::new(MathIdentifier::normal('あ'))), r#"<mi mathvariant="normal">あ</mi>"#),
        (MathML::Letter(Box::new(MathIdentifier::normal('啊'))), r#"<mi mathvariant="normal">啊</mi>"#),
        (MathML::Row(vec![MathML::Operator('+'), MathML::Operator('-')]), r"<mrow><mo>+</mo><mo>-</mo></mrow>"),
    ];
    for (problem, answer) in problems.iter() {
        assert_eq!(&format!("{}", problem), answer);
    }
}

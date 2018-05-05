#[test]
fn ready() {
    println!("it works!")
}

use mathml_core::{MathFenced, MathFraction, MathIdentifier, MathML, MathNumber};

#[test]
fn node_display() {
    let problems = vec![
        (MathML::Number(Box::new(MathNumber::from(3.14))), "<mn>3.14</mn>"),
        (MathML::Identifier(Box::new(MathIdentifier::italic('a'))), "<mi>a</mi>"),
        (MathML::Identifier(Box::new(MathIdentifier::italic('α'))), "<mi>α</mi>"),
        (MathML::Identifier(Box::new(MathIdentifier::italic('&'))), "<mi>&amp;</mi>"),
        (MathML::Identifier(Box::new(MathIdentifier::normal('あ'))), r#"<mi mathvariant="normal">あ</mi>"#),
        (MathML::Identifier(Box::new(MathIdentifier::normal('啊'))), r#"<mi mathvariant="normal">啊</mi>"#),
        // (MathML::Row(vec![MathML::Operator('+'), MathML::Operator('-')]), r"<mrow><mo>+</mo><mo>-</mo></mrow>"),
    ];
    for (problem, answer) in problems.iter() {
        assert_eq!(&format!("{}", problem), answer);
    }
}

#[test]
fn test() {
    let math = MathFenced::new(
        vec![MathFraction::new(1, 2).into(), MathNumber::from(2.1).into(), MathNumber::from(3.1).into()],
        '<',
        '>',
    )
    .with_separators("&#");
    println!("{}", math)
}

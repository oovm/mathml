#[test]
fn ready() {
    println!("it works!")
}

use mathml_core::{MathFenced, MathIdentifier, MathML};

#[test]
fn node_display() {
    let problems = vec![
        (MathML::number(3.14), "<mn>3.14</mn>"),
        (MathIdentifier::italic('a').into(), "<mi>a</mi>"),
        (MathIdentifier::italic('α').into(), "<mi>α</mi>"),
        (MathIdentifier::italic('&').into(), "<mi>&amp;</mi>"),
        (MathIdentifier::normal('あ').into(), r#"<mi mathvariant="normal">あ</mi>"#),
        (MathIdentifier::normal('啊').into(), r#"<mi mathvariant="normal">啊</mi>"#),
        // (MathML::Row(vec![MathML::Operator('+'), MathML::Operator('-')]), r"<mrow><mo>+</mo><mo>-</mo></mrow>"),
    ];
    for (problem, answer) in problems.iter() {
        assert_eq!(&format!("{}", problem), answer);
    }
}

#[test]
fn test() {
    let math = MathFenced::new(vec![MathML::fraction(1, 2), 2.into(), 3.14.into()], '<', '>').with_separators("&#");
    println!("{}", math)
}

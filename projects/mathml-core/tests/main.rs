#[test]
fn ready() {
    println!("it works!")
}


use mathml_core::MathML;

#[test]
fn node_display() {
    let problems = vec![
        (MathML::Number("3.14".to_owned()), "<mn>3.14</mn>"),
        (MathML::Letter('x', Variant::Italic), "<mi>x</mi>"),
        (MathML::Letter('α', Variant::Italic), "<mi>α</mi>"),
        (MathML::Letter('あ', Variant::Normal), r#"<mi mathvariant="normal">あ</mi>"#),
        (
            MathML::Row(vec![MathML::Operator('+'), MathML::Operator('-')]),
            r"<mrow><mo>+</mo><mo>-</mo></mrow>"
        ),
    ];
    for (problem, answer) in problems.iter() {
        assert_eq!(&format!("{}", problem), answer);
    }
}
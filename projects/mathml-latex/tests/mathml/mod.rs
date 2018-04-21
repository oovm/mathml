use mathml_latex::{parse_latex, LaTeXDefinition};
use pex::StopBecause;

#[test]
pub fn test_number() {
    assert_mathml(r"0", r"<mn>0</mn>").unwrap();
    assert_mathml(r"0.", r"<mn>0.</mn>").unwrap();
    assert_mathml(r".0", r"<mn>.0</mn>").unwrap();
    assert_mathml(r"0.0", r"<mn>0.0</mn>").unwrap();
}

#[test]
pub fn test_term() {
    assert_mathml(r"+ 0", r"<mrow><mo>+</mo><mn>0</mn></mrow>").unwrap();
    assert_mathml(r"1 + 1", r"<mrow><mn>1</mn><mo>+</mo><mn>1</mn></mrow>").unwrap();
    assert_mathml(r"a + b", r"<mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow>").unwrap();
    // assert_mathml(r"\frac{a}{b}", r"<mn>.0</mn>");
}

pub fn assert_mathml(source: &str, target: &str) -> Result<(), StopBecause> {
    let context = LaTeXDefinition::default();
    let mathml = parse_latex(source)?.as_mathml(&context);
    assert_eq!(format!("{}", mathml), target);
    Ok(())
}

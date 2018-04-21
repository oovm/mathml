use mathml_latex::{parse_latex, LaTeXDefinition};

#[test]
pub fn test_number() {
    assert_mathml(r"0", r"<mn>0</mn>");
    assert_mathml(r"0.1", r"<mn>0.1</mn>");
}

#[track_caller]
pub fn assert_mathml(source: &str, target: &str) {
    let context = LaTeXDefinition::default();
    let mathml = parse_latex(source).unwrap().as_mathml(&context);
    assert_eq!(format!("{}", mathml), target)
}

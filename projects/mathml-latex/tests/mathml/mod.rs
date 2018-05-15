use mathml_latex::{parse_latex, LaTeXEngine};
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
    assert_mathml(r"{1} + {1}", r"<mrow><mn>1</mn><mo>+</mo><mn>1</mn></mrow>").unwrap();
    assert_mathml(r"a + b", r"<mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow>").unwrap();
    assert_mathml(r"a ^ b", r"<msup><mi>a</mi><mi>b</mi></msup>").unwrap();
    // assert_mathml(r"{a ^ b} ^ c", r"<msup><mi>a</mi><mi>b</mi></msup>").unwrap();
}

#[test]
pub fn test_frac() {
    assert_mathml(r"\frac{a}{b}", r"<mfrac><mi>a</mi><mi>b</mi></mfrac>").unwrap();
    assert_mathml(r"\frac{a}{b}{c}", r"<mrow><mfrac><mi>a</mi><mi>b</mi></mfrac><mi>c</mi></mrow>").unwrap();
    assert_mathml(
        r"-\frac{p - q}{\Delta}",
        r#"<mrow><mo>−</mo><mfrac><mrow><mi>p</mi><mo>−</mo><mi>q</mi></mrow><mi mathvariant="normal">Δ</mi></mfrac></mrow>"#,
    )
    .unwrap();
}

#[test]
pub fn test_function() {
    assert_mathml(
        r"\sin{a}{b}",
        r#"<mrow><mi mathvariant="normal">sin</mi><mspace width="0.167rem"/><mi>a</mi><mi>b</mi></mrow>"#,
    )
    .unwrap();
    assert_mathml(
        r"\operatorname{a}{b}{c}",
        r#"<mrow><mi mathvariant="normal">a</mi><mspace width="0.167rem"/><mi>b</mi><mi>c</mi></mrow>"#,
    )
    .unwrap();
}

#[test]
pub fn test_matrix() {
    assert_mathml(
        r#"\begin{matrix} a & b \\ c & d \end{matrix}"#,
        "<mtable><mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd></mtr><mtr><mtd><mi>c</mi></mtd><mtd><mi>d</mi></mtd></mtr></mtable>",
    )
    .unwrap();
    assert_mathml(r#"\begin{bmatrix} a & b \\ c & d \end{bmatrix}"#, "<mrow><mo>[</mo><mtable><mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd></mtr><mtr><mtd><mi>c</mi></mtd><mtd><mi>d</mi></mtd></mtr></mtable><mo>]</mo></mrow>").unwrap();
    assert_mathml(r#"\begin{Bmatrix} a & b \\ c & d \end{Bmatrix}"#, "<mrow><mo>{</mo><mtable><mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd></mtr><mtr><mtd><mi>c</mi></mtd><mtd><mi>d</mi></mtd></mtr></mtable><mo>}</mo></mrow>").unwrap();
    assert_mathml(r#"\begin{vmatrix} a & b \\ c & d \end{vmatrix}"#, "<mrow><mo>|</mo><mtable><mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd></mtr><mtr><mtd><mi>c</mi></mtd><mtd><mi>d</mi></mtd></mtr></mtable><mo>|</mo></mrow>").unwrap();
    assert_mathml(r#"\begin{Vmatrix} a & b \\ c & d \end{Vmatrix}"#, "<mrow><mo>‖</mo><mtable><mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd></mtr><mtr><mtd><mi>c</mi></mtd><mtd><mi>d</mi></mtd></mtr></mtable><mo>‖</mo></mrow>").unwrap();
    assert_mathml(r#"\begin{cases} a & b & c\\ d & e & f \\ \end{cases}"#, r#"<mrow><mo>{</mo><mtable columnalign="left"><mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd><mtd><mi>c</mi></mtd></mtr><mtr><mtd><mi>d</mi></mtd><mtd><mi>e</mi></mtd><mtd><mi>f</mi></mtd></mtr><mtr><mtd></mtd></mtr></mtable></mrow>"#).unwrap();
}

pub fn assert_mathml(source: &str, target: &str) -> Result<(), StopBecause> {
    let context = LaTeXEngine::builtin();
    let mathml = parse_latex(source)?.as_mathml(&context);
    assert_eq!(format!("{}", mathml), target);
    Ok(())
}

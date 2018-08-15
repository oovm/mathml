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
    assert_refine(include_str!("continued_frac.tex"), include_str!("continued_frac.xml")).unwrap();
    assert_refine(include_str!("continued_dfrac.tex"), include_str!("continued_dfrac.xml")).unwrap();
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
    assert_refine(r#"\begin{bmatrix} a & b \\ c & d \end{bmatrix}"#, include_str!("bmatrix.xml")).unwrap();
    assert_refine(r#"\begin{Bmatrix} a & b \\ c & d \end{Bmatrix}"#, include_str!("bmatrix2.xml")).unwrap();
    assert_refine(r#"\begin{vmatrix} a & b \\ c & d \end{vmatrix}"#, include_str!("vmatrix.xml")).unwrap();
    assert_refine(r#"\begin{Vmatrix} a & b \\ c & d \end{Vmatrix}"#, include_str!("vmatrix2.xml")).unwrap();
    assert_refine(include_str!("matrix_nested.tex"), include_str!("matrix_nested.xml")).unwrap();
    assert_refine(include_str!("piece_cases.tex"), include_str!("piece_cases.xml")).unwrap();
}

pub fn assert_mathml(source: &str, target: &str) -> Result<(), StopBecause> {
    assert_eq!(render_mathml(source)?, target);
    Ok(())
}

#[track_caller]
pub fn render_mathml(source: &str) -> Result<String, StopBecause> {
    let context = LaTeXEngine::builtin();
    let mathml = parse_latex(source)?.as_mathml(&context);
    Ok(format!("{}", mathml))
}

#[track_caller]
pub fn assert_refine(source: &str, target: &str) -> Result<(), StopBecause> {
    assert_eq!(refine_string(&render_mathml(source)?), refine_string(target));
    Ok(())
}

pub fn refine_string(string: &str) -> String {
    let mut refined = String::with_capacity(string.len());
    for char in string.chars() {
        if char.is_whitespace() {
            continue;
        }
        refined.push(char);
    }
    refined
}

use super::*;

pub fn matrix<I>(rows: I) -> MathML
where
    I: IntoIterator<Item = MathML>,
{
    MathTable::matrix(rows).into()
}

// noinspection SpellCheckingInspection
pub fn bmatrix<I>(rows: I) -> MathML
where
    I: IntoIterator<Item = MathML>,
{
    MathRow::new(vec![MathML::operation("[").into(), MathTable::matrix(rows).into(), MathML::operation("]").into()]).into()
}

// noinspection SpellCheckingInspection
pub fn Bmatrix<I>(rows: I) -> MathML
where
    I: IntoIterator<Item = MathML>,
{
    MathRow::new(vec![MathML::operation("{").into(), MathTable::matrix(rows).into(), MathML::operation("}").into()]).into()
}

// noinspection SpellCheckingInspection
pub fn vmatrix<I>(rows: I) -> MathML
where
    I: IntoIterator<Item = MathML>,
{
    MathRow::new(vec![MathML::operation("|").into(), MathTable::matrix(rows).into(), MathML::operation("|").into()]).into()
}

/// Renders a matrix with double vertical bars.
///
/// # Arguments
///
/// * `rows`:
///
/// returns: MathML
///
/// # Examples
///
/// ```
/// # use mathml_core::helpers::Vmatrix;
/// ```
///
/// # Output
///
/// <math xmlns="http://www.w3.org/1998/Math/MathML" display="block"><mrow><mo>‖</mo><mtable><mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd></mtr><mtr><mtd><mi>c</mi></mtd><mtd><mi>d</mi></mtd></mtr></mtable><mo>‖</mo></mrow></math>
// noinspection SpellCheckingInspection
pub fn Vmatrix<I>(rows: I) -> MathML
where
    I: IntoIterator<Item = MathML>,
{
    MathRow::new(vec![MathML::operation("‖").into(), MathTable::matrix(rows).into(), MathML::operation("‖").into()]).into()
}

// noinspection SpellCheckingInspection
pub fn pmatrix<I>(rows: I) -> MathML
where
    I: IntoIterator<Item = MathML>,
{
    MathRow::new(vec![MathML::operation("(").into(), MathTable::matrix(rows).into(), MathML::operation(")").into()]).into()
}

// noinspection SpellCheckingInspection
pub fn Pmatrix<I>(rows: I) -> MathML
where
    I: IntoIterator<Item = MathML>,
{
    MathRow::new(vec![MathML::operation("⎛").into(), MathTable::matrix(rows).into(), MathML::operation("⎞").into()]).into()
}

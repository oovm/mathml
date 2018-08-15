use super::*;

// noinspection SpellCheckingInspection
/// Renders a matrix without vertical bars.
///
/// # Input
///
/// ```tex
/// \begin{matrix} a & b \\ c & d \end{matrix}
/// ```
///
/// # Output
#[doc = include_str!("vmatrix.xml")]
pub fn matrix<I>(rows: I) -> MathML
where
    I: IntoIterator<Item = MathML>,
{
    MathTable::matrix(rows).into()
}
// noinspection SpellCheckingInspection
/// Renders a small matrix without vertical bars.
///
/// # Input
///
/// ```tex
/// \begin{vmatrix} a & b \\ c & d \end{vmatrix}
/// ```
///
/// # Output
#[doc = include_str!("vmatrix.xml")]
pub fn smallmatrix<I>(rows: I) -> MathML
where
    I: IntoIterator<Item = MathML>,
{
    // mstyle scriptlevel="1"
    todo!()
}
// noinspection SpellCheckingInspection
/// Renders a matrix with brackets.
///
/// # Input
///
/// ```tex
/// \begin{bmatrix} a & b \\ c & d \end{bmatrix}
/// ```
///
/// # Output
#[doc = include_str!("bmatrix.xml")]
pub fn bmatrix<I>(rows: I) -> MathML
where
    I: IntoIterator<Item = MathML>,
{
    MathRow::new(vec![MathML::operation("[").into(), MathTable::matrix(rows).into(), MathML::operation("]").into()]).into()
}
// noinspection SpellCheckingInspection
/// Renders a matrix with curly brackets.
///
/// # Input
///
/// ```tex
/// \begin{Bmatrix} a & b \\ c & d \end{Bmatrix}
/// ```
///
/// # Output
#[doc = include_str!("bmatrix2.xml")]
pub fn Bmatrix<I>(rows: I) -> MathML
where
    I: IntoIterator<Item = MathML>,
{
    MathRow::new(vec![MathML::operation("{").into(), MathTable::matrix(rows).into(), MathML::operation("}").into()]).into()
}
// noinspection SpellCheckingInspection
/// Renders a matrix with vertical bars.
///
/// # Input
///
/// ```tex
/// \begin{vmatrix} a & b \\ c & d \end{vmatrix}
/// ```
///
/// # Output
#[doc = include_str!("vmatrix.xml")]
pub fn vmatrix<I>(rows: I) -> MathML
where
    I: IntoIterator<Item = MathML>,
{
    MathRow::new(vec![MathML::operation("|").into(), MathTable::matrix(rows).into(), MathML::operation("|").into()]).into()
}
// noinspection SpellCheckingInspection
/// Renders a matrix with double vertical bars.
///
///
/// # Input
///
/// ```tex
/// \begin{Vmatrix} a & b \\ c & d \end{Vmatrix}
/// ```
///
/// # Output
#[doc = include_str!("vmatrix2.xml")]
pub fn Vmatrix<I>(items: I) -> MathML
where
    I: IntoIterator<Item = MathML>,
{
    MathRow::new(vec![MathML::operation("‖").into(), MathTable::matrix(items).into(), MathML::operation("‖").into()]).into()
}

// noinspection SpellCheckingInspection
/// Renders a matrix with parentheses.
///
/// # Input
///
/// ```tex
/// \begin{pmatrix} a & b \\ c & d \end{pmatrix}
/// ```
///
/// # Output
#[doc = include_str!("pmatrix.xml")]
pub fn pmatrix<I>(items: I) -> MathML
where
    I: IntoIterator<Item = MathML>,
{
    MathRow::new(vec![MathML::operation("(").into(), MathTable::matrix(items).into(), MathML::operation(")").into()]).into()
}

// noinspection SpellCheckingInspection
/// Renders a piecewise function.
///
/// # Input
///
/// ```tex
/// \begin{vmatrix} a & b \\ c & d \end{vmatrix}
/// ```
///
/// # Output
#[doc = include_str!("cases.xml")]
pub fn cases<I>(items: I) -> MathML
where
    I: IntoIterator<Item = MathML>,
{
    MathRow::new(vec![MathML::operation("{").into(), MathTable::matrix(items).with_attribute("columnalign", "left").into()])
        .into()
}

use super::*;

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
pub fn matrix<I>(rows: I) -> MathML
where
    I: IntoIterator<Item = MathML>,
{
    MathTable::matrix(rows).into()
}

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
// noinspection SpellCheckingInspection
pub fn smallmatrix<I>(rows: I) -> MathML
where
    I: IntoIterator<Item = MathML>,
{
    // mstyle scriptlevel="1"
    todo!()
}

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
// noinspection SpellCheckingInspection
pub fn bmatrix<I>(rows: I) -> MathML
where
    I: IntoIterator<Item = MathML>,
{
    MathRow::new(vec![MathML::operation("[").into(), MathTable::matrix(rows).into(), MathML::operation("]").into()]).into()
}

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
// noinspection SpellCheckingInspection
pub fn Bmatrix<I>(rows: I) -> MathML
where
    I: IntoIterator<Item = MathML>,
{
    MathRow::new(vec![MathML::operation("{").into(), MathTable::matrix(rows).into(), MathML::operation("}").into()]).into()
}

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
// noinspection SpellCheckingInspection
pub fn vmatrix<I>(rows: I) -> MathML
where
    I: IntoIterator<Item = MathML>,
{
    MathRow::new(vec![MathML::operation("|").into(), MathTable::matrix(rows).into(), MathML::operation("|").into()]).into()
}

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
// noinspection SpellCheckingInspection
pub fn Vmatrix<I>(items: I) -> MathML
where
    I: IntoIterator<Item = MathML>,
{
    MathRow::new(vec![MathML::operation("‖").into(), MathTable::matrix(items).into(), MathML::operation("‖").into()]).into()
}

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
// noinspection SpellCheckingInspection
pub fn pmatrix<I>(items: I) -> MathML
where
    I: IntoIterator<Item = MathML>,
{
    MathRow::new(vec![MathML::operation("(").into(), MathTable::matrix(items).into(), MathML::operation(")").into()]).into()
}

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
// noinspection SpellCheckingInspection
pub fn Pmatrix<I>(items: I) -> MathML
where
    I: IntoIterator<Item = MathML>,
{
    MathRow::new(vec![MathML::operation("⎛").into(), MathTable::matrix(items).into(), MathML::operation("⎞").into()]).into()
}

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
// noinspection SpellCheckingInspection
pub fn cases<I>(items: I) -> MathML
where
    I: IntoIterator<Item = MathML>,
{
    MathRow::new(vec![MathML::operation("{").into(), MathTable::matrix(items).with_attribute("columnalign", "left").into()])
        .into()
}

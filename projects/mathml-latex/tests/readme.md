latex2mathml

Provides a functionality to convert LaTeX math equations to MathML representation.
This crate is implemented in pure Rust, so it works for all platforms including WebAssembly.

# Supported LaTeX commands

- Numbers, e.g. `0`, `3.14`, ...
- ASCII and Greek (and more) letters, e.g. `x`, `\alpha`, `\pi`, `\aleph`, ...
- Symbols, e.g., `\infty`, `\dagger`, `\angle`, `\Box`, `\partial`, ...
- Binary relations, e.g. `=`, `>`, `<`, `\ll`, `:=`, ...
- Binary operations, e.g. `+`. `-`, `*`, `/`, `\times`, `\otimes`, ...
- Basic LaTeX commands, e.g. `\sqrt`, `\frac`, `\sin`, `\binom`, ...
- Parentheses, e.g., `\left\{ .. \middle| .. \right]`, ...
- Integrals, e.g., `\int_0^\infty`, `\iint`, `\oint`, ...
- Big operators, e.g., `\sum`, `\prod`, `\bigcup_{i = 0}^\infty`, ...
- Limits and overset/underset, e.g., `\lim`, `\overset{}{}`, `\overbrace{}{}`, ...
- Font styles, e.g. `\mathrm`, `\mathbf`, `\bm`, `\mathit`, `\mathsf`, `\mathscr`, `\mathbb`, `\mathfrak`, `\texttt`.
    - MathML lacks calligraphic mathvariant: https://github.com/mathml-refresh/mathml/issues/61
- White spaces, e.g., `\!`, `\,`, `\:`, `\;`, `\ `, `\quad`, `\qquad`.
- Matrix, e.g. `\begin{matrix}`, `\begin{pmatrix}`, `\begin{bmatrix}`, `\begin{vmatrix}`.
- Multi-line equation `\begin{align}` (experimental).
- Feynman slash notation: `\slashed{\partial}`.

## Unsupported LaTeX commands

- New line `\\`, except for ones in a matrix or align environment.
- Alignment `&`, except for ones in a matrix or align environment.
- Complicated sub/superscripts (`<mmultiscripts>`).


# Usage

Main functions of this crate are  [`latex_to_mathml`](./fn.latex_to_mathml.html) and
[`replace`](./fn.replace.html).

```rust
use latex2mathml::{latex_to_mathml, DisplayStyle};

let latex = r#"\erf ( x ) = \frac{ 2 }{ \sqrt{ \pi } } \int_0^x e^{- t^2} \, dt"#;
let mathml = latex_to_mathml(latex, DisplayStyle::Block).unwrap();
println!("{}", mathml);
```

For converting a document including LaTeX equations, the function [`replace`](./fn.replace.html)
may be useful.

```rust
let latex = r#"The error function $\erf ( x )$ is defined by
$$\erf ( x ) = \frac{ 2 }{ \sqrt{ \pi } } \int_0^x e^{- t^2} \, dt .$$"#;

let mathml = latex2mathml::replace(latex).unwrap();
println!("{}", mathml);
```

If you want to transform the equations in a directory recursively, the function
[`convert_html`](./fn.convert_html.html) is useful.

```rust
use latex2mathml::convert_html;

convert_html("./target/doc").unwrap();
```

For more examples and list of supported LaTeX commands, please check
[`examples/equations.rs`](https://github.com/osanshouo/latex2mathml/blob/master/examples/equations.rs)
and [`examples/document.rs`](https://github.com/osanshouo/latex2mathml/blob/master/examples/document.rs).

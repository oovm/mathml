use super::attribute::{Accent, Variant};
use crate::ast::DisplayStyle;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Illegal(char),
    EOF,
    Begin,
    End,
    Ampersand,
    NewLine,
    Left,
    Right,
    Middle,
    Paren(&'static str),
    LBrace,
    RBrace,
    Frac,
    Underscore,
    Circumflex,
    Binom(Option<DisplayStyle>),
    Overset,
    Underset,
    Overbrace(char),
    Underbrace(char),
    Sqrt,
    Integral(char),
    Lim(&'static str),
    Space(f32),
    Style(Variant),
    Big(&'static str),
    Over(char, Accent),
    Under(char, Accent),
    Operator(char),
    BigOp(char),
    Letter(char, Variant),
    Number(String),
    Function(&'static str),
    OperatorName,
    Slashed,
    Text,
    Command(String),
}

impl Token {
    pub(crate) fn acts_on_a_digit(&self) -> bool {
        match self {
            Token::Sqrt | Token::Frac | Token::Binom(_) | Token::Style(_) => true,
            _ => false,
        }
    }

    pub fn from_command(command: &str) -> Token {
        match command {
            "mathrm" => Token::Style(Variant::Normal),
            "textit" => Token::Style(Variant::Italic),
            "mathit" => Token::Style(Variant::Italic),
            "textbf" => Token::Style(Variant::Bold),
            "mathbf" => Token::Style(Variant::Bold),
            "bm" => Token::Style(Variant::BoldItalic),
            "symbf" => Token::Style(Variant::BoldItalic),
            "mathbb" => Token::Style(Variant::DoubleStruck),
            "mathfrak" => Token::Style(Variant::Fraktur),
            "mathscr" => Token::Style(Variant::Script),
            "mathsf" => Token::Style(Variant::SansSerif),
            "texttt" => Token::Style(Variant::Monospace),
            "boldsymbol" => Token::Style(Variant::BoldItalic),
            "text" => Token::Text,
            "sqrt" => Token::Sqrt,
            "frac" => Token::Frac,
            "left" => Token::Left,
            "right" => Token::Right,
            "middle" => Token::Middle,
            "begin" => Token::Begin,
            "end" => Token::End,
            "\\" => Token::NewLine,
            "binom" => Token::Binom(None),
            "tbinom" => Token::Binom(Some(DisplayStyle::Inline)),
            "dbinom" => Token::Binom(Some(DisplayStyle::Block)),
            "overset" => Token::Overset,
            "underset" => Token::Underset,
            "overbrace" => Token::Overbrace('\u{23de}'),
            "underbrace" => Token::Underbrace('\u{23df}'),
            "overparen" => Token::Overbrace('\u{23dc}'),
            "underparen" => Token::Underbrace('\u{23dd}'),
            "overbracket" => Token::Overbrace('\u{23b4}'),
            "underbracket" => Token::Underbrace('\u{23b5}'),
            "langle" => Token::Paren("&lang;"),
            "rangle" => Token::Paren("&rang;"),
            "{" => Token::Paren("{"),
            "}" => Token::Paren("}"),
            "lceil" => Token::Paren("⌈"),
            "rceil" => Token::Paren("⌉"),
            "lfloor" => Token::Paren("⌊"),
            "rfloor" => Token::Paren("⌋"),
            "lgroup" => Token::Paren("⦗"),
            "rgroup" => Token::Paren("⦘"),
            "llbracket" => Token::Paren("⟦"),
            "rrbracket" => Token::Paren("⟧"),
            "lim" => Token::Lim("lim"),
            "liminf" => Token::Lim("lim inf"),
            "limsup" => Token::Lim("lim sup"),
            "min" => Token::Lim("min"),
            "max" => Token::Lim("max"),
            "inf" => Token::Lim("inf"),
            "sup" => Token::Lim("sup"),
            "int" => Token::Integral('∫'),
            "iint" => Token::Integral('∬'),
            "iiint" => Token::Integral('∭'),
            "oint" => Token::Integral('∮'),
            "dot" => Token::Over('\u{02d9}', Accent::True),
            "ddot" => Token::Over('¨', Accent::True),
            "bar" => Token::Over('¯', Accent::True),
            "hat" => Token::Over('^', Accent::True),
            "check" => Token::Over('ˇ', Accent::True),
            "breve" => Token::Over('˘', Accent::True),
            "acute" => Token::Over('´', Accent::True),
            "grave" => Token::Over('`', Accent::True),
            "tilde" => Token::Over('~', Accent::True),
            "vec" => Token::Over('→', Accent::True),
            "overline" => Token::Over('_', Accent::True),
            "underline" => Token::Under('_', Accent::True),
            "widehat" => Token::Over('^', Accent::True),
            "widetilde" => Token::Over('~', Accent::True),
            "overrightarrow" => Token::Over('→', Accent::True),
            "overleftarrow" => Token::Over('←', Accent::True),
            "sum" => Token::BigOp('∑'),
            "prod" => Token::BigOp('∏'),
            "coprod" => Token::BigOp('∐'),
            "bigcap" => Token::BigOp('⋂'),
            "bigcup" => Token::BigOp('⋃'),
            "bigsqcup" => Token::BigOp('⨆'),
            "bigvee" => Token::BigOp('⋁'),
            "bigwedge" => Token::BigOp('⋀'),
            "bigodot" => Token::BigOp('⨀'),
            "bitotimes" => Token::BigOp('⨂'),
            "bigoplus" => Token::BigOp('⨁'),
            "biguplus" => Token::BigOp('⨄'),
            "bigl" => Token::Big("1.2em"),
            "bigr" => Token::Big("1.2em"),
            "Bigl" => Token::Big("1.623em"),
            "Bigr" => Token::Big("1.623em"),
            "biggl" => Token::Big("2.047em"),
            "biggr" => Token::Big("2.047em"),
            "Biggl" => Token::Big("2.470em"),
            "Biggr" => Token::Big("2.470em"),
            // <math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
            //   <semantics>
            //     <mrow>
            //         <mi>a</mi>
            //         <mrow class="MJX-TeXAtom-OPEN"><mo maxsize="2.470em" minsize="2.470em">(</mo></ mrow>
            //         <mi>b</mi>
            //         <mrow class="MJX-TeXAtom-OPEN"><mo maxsize="2.047em" minsize="2.047em">(</mo></mrow>
            //         <mi>c</mi>
            //         <mrow class="MJX-TeXAtom-OPEN"><mo maxsize="1.623em" minsize="1.623em">(</mo></mrow>
            //         <mi>d</mi>
            //         <mrow class="MJX-TeXAtom-OPEN"><mo maxsize="1.2em" minsize="1.2em">(</mo></mrow>
            //         <mi>e</mi>
            //         <mo stretchy="false">(</mo>
            //         <mi>f</mi>
            //         <mo>+</mo>
            //         <mi>g</mi>
            //         <mo stretchy="false">)</mo>
            //         <mrow class="MJX-TeXAtom-CLOSE"><mo maxsize="1.2em" minsize="1.2em">)</mo></mrow>
            //         <mrow class="MJX-TeXAtom-CLOSE"><mo maxsize="1.623em" minsize="1.623em">)</mo></mrow>
            //         <mrow class="MJX-TeXAtom-CLOSE"><mo maxsize="2.047em" minsize="2.047em">)</mo></mrow>
            //         <mrow class="MJX-TeXAtom-CLOSE"><mo maxsize="2.470em" minsize="2.470em">)</mo></mrow>
            //   </semantics>
            // </math>
            "slashed" => Token::Slashed,
            command => Token::Command(command.to_owned()),
        }
    }
}

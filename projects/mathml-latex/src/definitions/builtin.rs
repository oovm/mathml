use super::*;

impl Default for LaTeXEngine {
    fn default() -> Self {
        Self {
            functions: Default::default(),
            operators: Default::default(),
            spaces: Default::default(),
            letters: Default::default(),
        }
    }
}

impl LaTeXEngine {
    pub fn builtin() -> Self {
        let mut empty = Self::default();
        empty.add_builtin_operators();
        empty.add_builtin_functions();
        empty.add_builtin_letters();
        empty.add_builtin_space();
        empty
    }
}

impl LaTeXEngine {
    pub fn get_function(&self, name: &str) -> Option<&str> {
        Some(self.functions.get(name)?.as_str())
    }
    pub fn add_function<K, V>(&mut self, key: K, value: V)
    where
        K: ToString,
        V: ToString,
    {
        self.functions.insert(key.to_string(), value.to_string());
    }
    pub fn mut_functions<I>(&mut self) -> &mut BTreeMap<String, String> {
        &mut self.functions
    }
    fn add_builtin_functions(&mut self) {
        macro_rules! add_function {
            ($($name:literal => $symbol:literal),* $(,)?) => {
                $(
                    self.functions.insert($name.to_string(), $symbol.to_string());
                )*
            };
        }
        add_function! {
            "sin" => "sin",
            "cos" => "cos",
            "tan" => "tan",
            "csc" => "csc",
            "sec" => "sec",
            "cot" => "cot",
            "arcsin" => "arcsin",
            "arccos" => "arccos",
            "arctan" => "arctan",
            "sinh" => "sinh",
            "cosh" => "cosh",
            "tanh" => "tanh",
            "coth" => "coth",
            "exp" => "exp",
            "ln" => "ln",
            "log" => "log",
            "erf" => "erf",
            "erfc" => "erfc",
            "arg" => "arg",
            "gcd" => "gcd",
            "lcm" => "lcm",
            "min" => "min",
            "max" => "max",
        }
    }
}

impl LaTeXEngine {
    pub fn get_operator(&self, name: &str) -> Option<&str> {
        Some(self.operators.get(name)?.as_str())
    }
    fn add_builtin_operators(&mut self) {
        macro_rules! add_operator {
            ($($name:literal => $symbol:literal),* $(,)?) => {
                $(
                    self.operators.insert($name.to_string(), $symbol.to_string());
                )*
            };
        }

        add_operator! {
            "S" => "§",
            "P" => "¶",
            "%" => "%",
            "_" => "_",
            "&" => "&",
            "#" => "#",
            "$" => "$",
            "times"  => "×",
            "oplus"  => "⊕",
            "otimes" => "⊗",
            "odot"   => "⊙",
            "cup"    => "∪",
            "cap"    => "∩",
            "sqcup"  => "⊔",
            "sqcap"  => "⊓",
            "vee"    => "∨",
            "wedge"  => "∧",
            "setminus" => "∖",
            "uplus"  => "⊎",
            "amalg"  => "⨿",
            "bigcup" => "⋃",
            "bigcap" => "⋂",
        }
    }
}

impl LaTeXEngine {
    pub fn get_letters(&self, name: &str) -> Option<&str> {
        Some(self.letters.get(name)?.as_str())
    }
    fn add_builtin_letters(&mut self) {
        macro_rules! add_letter {
            ($($name:literal => $symbol:literal),* $(,)?) => {
                $(
                    self.letters.insert($name.to_string(), $symbol.to_string());
                )*
            };
        }

        add_letter! {
            "Alpha"  => "Α",
            "alpha"  => "α",
            "Beta"   => "Β",
            "beta"   => "β",
            "Gamma"  => "Γ",
            "gamma"  => "γ",
            "Delta"  => "Δ",
            "delta"  => "δ",
            "Epsilon"  => "Ε",
            "epsilon"  => "ε",
            "Zeta"  => "Ζ",
            "zeta"  => "ζ",
            "Eta"  => "Η",
            "eta"  => "η",
            "Theta"  => "Θ",
            "theta"  => "θ",
            "Iota"  => "Ι",
            "iota"  => "ι",
            "Kappa"  => "Κ",
            "kappa"  => "κ",
            "Lambda"  => "Λ",
            "lambda"  => "λ",
            "Mu"  => "Μ",
            "mu"  => "μ",
            "Nu"  => "Ν",
            "nu"  => "ν",
            "Xi"  => "Ξ",
            "xi"  => "ξ",
            "Omicron"  => "Ο",
            "omicron"  => "ο",
            "Pi"  => "Π",
            "pi"  => "π",
            "Rho"  => "Ρ",
            "rho"  => "ρ",
            "Sigma"  => "Σ",
            "sigma"  => "σ",
            "Tau"  => "Τ",
            "tau"  => "τ",
            "Upsilon"  => "Υ",
            "upsilon"  => "υ",
            "Phi"  => "Φ",
            "phi"  => "φ",
            "Chi"  => "Χ",
            "chi"  => "χ",
            "Psi"  => "Ψ",
            "psi"  => "ψ",
            "Omega"  => "Ω",
            "omega"  => "ω",
            //
            "aleph" => "ℵ",
            "beth" => "ℶ",
            "gimel" => "ℷ",
            "daleth" => "ℸ",
            "A" => "Å",
            "a" => "å",
            "AE" => "Æ",
            "ae" => "æ",
            "DH" => "Ð",
            "dh" => "ð",
            "dj" => "đ",
            "L" => "Ł",
            "l" => "ł",
            "NG" => "Ŋ",
            "ng" => "ŋ",
            "O" => "Ø",
            "o" => "ø",
            "OE" => "Œ",
            "oe" => "œ",
            "ss" => "ß",
            "TH" => "Þ",
            "th" => "þ",
            "imath" => "ı",
            "jmath" => "ȷ",
            "ell" => "ℓ",
            "hbar" => "ℏ",
            "hslash" => "ℏ",
            "infty" => "∞",
            "mho" => "℧",
            "Finv" => "Ⅎ",
            "Re" => "ℜ",
            "Im" => "ℑ",
            "wp" => "℘",
            "alef" => "ℵ",
            "alefsym" => "ℵ",
            "real" => "ℜ",
            "partial" => "∂",
            "prime" => "′",
            "emptyset" => "∅",
            "clubs" => "♣",
        }
    }
}

impl LaTeXEngine {
    pub fn get_space(&self, name: &str) -> Option<f32> {
        Some(*self.spaces.get(name)?)
    }
    pub fn add_space(&mut self, name: &str, value: f32) {
        self.spaces.insert(name.to_string(), value);
    }
    fn add_builtin_space(&mut self) {
        self.spaces.insert("!".to_string(), -3.0 / 18.0);
        self.spaces.insert(",".to_string(), 3.0 / 18.0);
        self.spaces.insert(":".to_string(), 4.0 / 18.0);
        self.spaces.insert(";".to_string(), 5.0 / 18.0);
        self.spaces.insert(" ".to_string(), 1.0);
        self.spaces.insert("quad".to_string(), 1.0);
        self.spaces.insert("qquad".to_string(), 2.0);
    }
}

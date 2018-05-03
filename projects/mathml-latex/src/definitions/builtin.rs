use super::*;

impl Default for LaTeXEngine {
    fn default() -> Self {
        let mut empty = Self { functions: Default::default(), operators: Default::default() };
        empty.add_builtin_operators();
        empty.add_builtin_functions();
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
    pub fn match_operator(&self, name: &str) -> Option<&str> {
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
        }
    }
}

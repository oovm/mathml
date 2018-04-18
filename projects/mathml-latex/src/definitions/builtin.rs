use super::*;

impl LaTeXDefinition {
    fn empty() -> Self {
        Self { functions: BTreeSet::new() }
    }
}

impl LaTeXDefinition {
    pub fn get_functions(&self) -> &BTreeSet<String> {
        &self.functions
    }
    pub fn add_functions<I>(&mut self, functions: I)
    where
        I: IntoIterator<Item = String>,
    {
        self.functions.extend(functions)
    }
    pub fn set_functions<I>(&mut self, functions: I)
    where
        I: IntoIterator<Item = String>,
    {
        self.functions = functions.into_iter().collect()
    }
    fn builtin_functions() -> BTreeSet<String> {
        const BUILTIN_FUNCTIONS: &[&str] = &[
            "sin", "cos", "tan", "csc", "sec", "cot", "arcsin", "arccos", "arctan", "sinh", "cosh", "tanh", "coth", "exp",
            "ln", "log", "erf", "erfc", "arg", "gcd", "lcm", "min", "max", "lim", "liminf", "limsup", "inf", "sup",
        ];
        BTreeSet::from_iter(BUILTIN_FUNCTIONS.into_iter().map(|s| s.to_string()))
    }
}

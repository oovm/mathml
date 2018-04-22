use std::collections::BTreeSet;
mod builtin;

pub struct LaTeXEngine {
    functions: BTreeSet<String>,
    operators: BTreeSet<String>,
}

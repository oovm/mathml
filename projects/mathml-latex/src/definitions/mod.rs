use std::collections::BTreeSet;
mod builtin;

pub struct LaTeXDefinition {
    functions: BTreeSet<String>,
}

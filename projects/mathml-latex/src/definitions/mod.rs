use std::collections::BTreeMap;
mod builtin;

pub struct LaTeXEngine {
    functions: BTreeMap<String, String>,
    operators: BTreeMap<String, String>,
}

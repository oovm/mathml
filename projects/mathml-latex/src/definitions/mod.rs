use std::collections::BTreeMap;
mod builtin;

#[derive(Clone, Debug)]
pub struct LaTeXEngine {
    functions: BTreeMap<String, String>,
    operators: BTreeMap<String, String>,
    spaces: BTreeMap<String, f32>,
    letters: BTreeMap<String, String>,
}

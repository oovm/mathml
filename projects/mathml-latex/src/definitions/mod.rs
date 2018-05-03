use std::collections::{BTreeMap, BTreeSet};
mod builtin;

pub struct LaTeXEngine {
    functions: BTreeMap<String, String>,
    operators: BTreeMap<String, String>,
}

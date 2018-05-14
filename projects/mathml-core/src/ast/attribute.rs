use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Accent {
    True,
    False,
}

impl Display for Accent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Accent::True => write!(f, "true"),
            Accent::False => write!(f, "false"),
        }
    }
}

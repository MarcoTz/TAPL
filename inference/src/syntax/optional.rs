use super::Term;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Nothing;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Something {
    pub term: Box<Term>,
}

impl From<Nothing> for Term {
    fn from(not: Nothing) -> Term {
        Term::Nothing(not)
    }
}

impl From<Something> for Term {
    fn from(some: Something) -> Term {
        Term::Something(some)
    }
}

impl fmt::Display for Nothing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Nothing")
    }
}

impl fmt::Display for Something {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Some({})", self.term)
    }
}

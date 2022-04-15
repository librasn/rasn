use super::*;

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Prefix {
    pub encoding: Option<String>,
    pub class: Option<Class>,
    pub kind: TagKind,
    pub number: Number,
}

impl Prefix {
    pub fn new(
        encoding: Option<String>,
        kind: TagKind,
        class: Option<Class>,
        number: Number,
    ) -> Self {
        Self {
            encoding,
            class,
            kind,
            number,
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Variation)]
pub enum TagKind {
    Implicit,
    Explicit,
    Environment,
}

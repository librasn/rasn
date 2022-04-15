use std::fmt;

use variation::Variation;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Import {
    path: Vec<String>,
    visibility: Visibility,
}

impl Import {
    pub fn new(visibility: Visibility, path: Vec<String>) -> Self {
        Self { visibility, path }
    }
}

impl fmt::Display for Import {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{vis}use {path};",
            vis = self.visibility,
            path = self.path.join("::")
        )
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Variation)]
pub enum Visibility {
    Public,
    Crate,
    Private,
}

impl fmt::Display for Visibility {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let visibility = match self {
            Visibility::Public => "pub ",
            Visibility::Crate => "pub(crate) ",
            Visibility::Private => "",
        };

        visibility.fmt(f)
    }
}

impl Default for Visibility {
    fn default() -> Self {
        Visibility::Private
    }
}

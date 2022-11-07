use alloc::borrow::Cow;

use crate::types::{Tag, TagTree};

#[derive(Debug, Clone)]
pub struct Variants {
    fields: Vec<Tag>,
}

impl Variants {
    pub fn new(fields: Cow<'static, [TagTree]>) -> Self {
        let fields = (&*fields).iter().flat_map(|tree| {
            fn flatten_tree(tree: &TagTree) -> Vec<Tag> {
                match tree {
                    TagTree::Leaf(tag) => vec![*tag],
                    TagTree::Choice(tree) => {
                        tree.iter().flat_map(flatten_tree).collect()
                    }
                }
            }

            flatten_tree(tree)
        })
        .collect();

        Self { fields }
    }

    pub const fn empty() -> Self {
        Self { fields: Vec::new() }
    }

    pub fn from_static(fields: &'static [TagTree]) -> Self {
        Self::new(Cow::Borrowed(fields))
    }
}

impl From<Cow<'static, [TagTree]>> for Variants {
    fn from(fields: Cow<'static, [TagTree]>) -> Self {
        Self::new(fields)
    }
}

impl core::ops::Deref for Variants {
    type Target = [Tag];

    fn deref(&self) -> &Self::Target {
        &self.fields
    }
}

#![allow(warnings)]

#[macro_use]
extern crate log;

mod codegen;
mod parser;
mod registry;
mod semantics;

use std::{fs, path::PathBuf};

use self::{codegen::*, parser::Parser, semantics::*};

pub type Result<T> = std::result::Result<T, failure::Error>;

pub struct NotationCompiler {
    path: PathBuf,
    dependencies: Option<PathBuf>,
}

impl NotationCompiler {
    pub fn new<I: Into<PathBuf>>(path: I) -> Self {
        Self {
            path: path.into(),
            dependencies: None,
        }
    }

    pub fn dependencies<I: Into<PathBuf>>(mut self, path: I) -> Self {
        self.dependencies = Some(path.into());
        self
    }

    pub fn build(self) -> Result<String> {
        let source = fs::read_to_string(&self.path)?;
        let ast = Parser::parse(&source)?;

        let mut fixed_tree = SemanticChecker::new(ast);
        fixed_tree.build()?;

        let mut output = Vec::new();

        CodeGenerator::<Vec<u8>, Rust>::new(fixed_tree, &mut output)
            .generate()?;

        Ok(String::from_utf8(output).unwrap())
    }
}

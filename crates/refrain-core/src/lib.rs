//! refrain-core: Differential Refrain Engine core library.
//!
//! This crate defines the AST, public types, and parser entry points for the
//! Refrain DSL. The DSL is an S-expression language describing time-pattern
//! refrains and their compositional transformations.

pub mod ast;
pub mod error;
pub mod parser;

pub use ast::{Refrain, Pattern, Op};
pub use error::{RefrainError, Result};
pub use parser::parse;

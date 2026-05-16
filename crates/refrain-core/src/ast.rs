//! Refrain AST.
//!
//! A `Refrain` is a named time-pattern with three structural stages —
//! `territorialize`, `deterritorialize`, `reterritorialize` — each containing
//! a `Pattern` tree built from primitive `Op` nodes.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Refrain {
    pub name: String,
    pub territorialize: Option<Pattern>,
    pub deterritorialize: Option<Pattern>,
    pub reterritorialize: Option<Pattern>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Pattern {
    Op(Op),
    Seq(Vec<Pattern>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Op {
    Note { pitch: String, dur: String },
    Loop { count: u32, body: Box<Pattern> },
    Diff { x: String, t: String },
    Quotient { rels: Vec<String> },
    Sym(String),
}

impl Refrain {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            territorialize: None,
            deterritorialize: None,
            reterritorialize: None,
        }
    }
}

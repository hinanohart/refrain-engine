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
    Call { head: String, args: Vec<Pattern> },
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

    pub fn stages(&self) -> impl Iterator<Item = (StageKind, &Pattern)> {
        let t = self
            .territorialize
            .as_ref()
            .map(|p| (StageKind::Territorialize, p));
        let d = self
            .deterritorialize
            .as_ref()
            .map(|p| (StageKind::Deterritorialize, p));
        let r = self
            .reterritorialize
            .as_ref()
            .map(|p| (StageKind::Reterritorialize, p));
        t.into_iter().chain(d).chain(r)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StageKind {
    Territorialize,
    Deterritorialize,
    Reterritorialize,
}

impl StageKind {
    pub fn as_str(self) -> &'static str {
        match self {
            StageKind::Territorialize => "territorialize",
            StageKind::Deterritorialize => "deterritorialize",
            StageKind::Reterritorialize => "reterritorialize",
        }
    }
}

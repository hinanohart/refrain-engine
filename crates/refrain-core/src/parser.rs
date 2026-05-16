//! Parser placeholder for Phase 1; full chumsky parser lands in Phase 3.

use crate::ast::Refrain;
use crate::error::{RefrainError, Result};

pub fn parse(_src: &str) -> Result<Refrain> {
    Err(RefrainError::Parse(
        "parser not yet implemented (Phase 3)".into(),
    ))
}

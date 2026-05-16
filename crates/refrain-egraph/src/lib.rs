//! refrain-egraph: egglog-based normalization and extraction for Refrain ASTs.
//!
//! Provides a thin Rust wrapper over the egglog rewrite engine plus the
//! Refrain-specific rule set and an extraction cost model.
//! Full bindings land in Phase 4.

use refrain_core::Refrain;
use refrain_core::Result;

pub struct Egraph {
    _todo_phase_4: (),
}

impl Egraph {
    pub fn new() -> Self {
        Self { _todo_phase_4: () }
    }

    pub fn normalize(&self, r: &Refrain) -> Result<Refrain> {
        Ok(r.clone())
    }
}

impl Default for Egraph {
    fn default() -> Self {
        Self::new()
    }
}

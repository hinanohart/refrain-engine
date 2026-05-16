//! refrain-rhizome: Loro CRDT bridge for collaborative refrain editing.
//!
//! Opt-in via `--features rhizome`. Bridges Loro's eg-walker event graph with
//! the egglog e-class identifier space using a two-layer HashMap mapping
//! table (causal_id ↔ eclass_id). Full implementation in Phase 10.

#[cfg(feature = "rhizome")]
pub mod bridge {
    pub struct Rhizome;
    impl Rhizome {
        pub fn new() -> Self {
            Self
        }
    }
    impl Default for Rhizome {
        fn default() -> Self {
            Self::new()
        }
    }
}

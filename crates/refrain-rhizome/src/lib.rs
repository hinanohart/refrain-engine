//! refrain-rhizome: Loro-CRDT-style collaborative editing for refrains.
//!
//! This crate ships the **bridge** data structure that maps between Loro's
//! eg-walker causal event graph and the e-class identifiers used by the
//! `refrain-egraph` normalizer. The bridge is implemented as a two-layer
//! HashMap:
//!
//! * `causal_to_eclass: CausalId -> EClassId`
//! * `eclass_to_causal: EClassId -> Vec<CausalId>`
//!
//! Union of e-classes preserves causal identities by repointing all
//! covering causal ids to the surviving class.
//!
//! The optional `rhizome` feature wires Loro itself in (planned for v0.2
//! once Loro 1.0 ships stable; the current `1.0.0-beta.*` API can yank).
//! Without that feature the bridge is fully exercised by in-memory tests
//! using plain `u64` event ids.
//!
//! # Examples
//!
//! ```
//! use refrain_rhizome::RhizomeBridge;
//!
//! let mut b = RhizomeBridge::new();
//! let e0 = b.alloc_eclass();
//! let e1 = b.alloc_eclass();
//! b.insert(10, e0);
//! b.insert(11, e0);
//! b.insert(20, e1);
//!
//! assert_eq!(b.eclass_of(10), Some(e0));
//! assert_eq!(b.causal_events_of(e0).len(), 2);
//!
//! b.merge_classes(e0, e1);
//! // After merge, all three causal ids live in the lower-numbered class.
//! assert_eq!(b.eclass_of(10), b.eclass_of(20));
//! assert_eq!(b.causal_events_of(e0).len(), 3);
//! ```

use std::collections::HashMap;

pub type CausalId = u64;
pub type EClassId = u64;

/// Two-layer HashMap bridge between Loro causal-id space and egglog e-class
/// space.
#[derive(Debug, Default)]
pub struct RhizomeBridge {
    causal_to_eclass: HashMap<CausalId, EClassId>,
    eclass_to_causal: HashMap<EClassId, Vec<CausalId>>,
    next_eclass: u64,
}

impl RhizomeBridge {
    pub fn new() -> Self {
        Self::default()
    }

    /// Allocate a fresh, unused e-class id.
    pub fn alloc_eclass(&mut self) -> EClassId {
        let id = self.next_eclass;
        self.next_eclass += 1;
        id
    }

    /// Bind a causal id to an e-class. Idempotent for the same pair; if the
    /// causal id was previously bound elsewhere, the old binding is removed
    /// from the reverse map.
    pub fn insert(&mut self, causal: CausalId, eclass: EClassId) {
        if let Some(prev) = self.causal_to_eclass.insert(causal, eclass) {
            if prev != eclass {
                if let Some(v) = self.eclass_to_causal.get_mut(&prev) {
                    v.retain(|c| *c != causal);
                }
            }
        }
        self.eclass_to_causal
            .entry(eclass)
            .or_default()
            .push(causal);
    }

    pub fn eclass_of(&self, causal: CausalId) -> Option<EClassId> {
        self.causal_to_eclass.get(&causal).copied()
    }

    pub fn causal_events_of(&self, eclass: EClassId) -> &[CausalId] {
        self.eclass_to_causal
            .get(&eclass)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Merge two e-classes; the surviving id is `min(a, b)`. Causal ids in
    /// the dropped class are repointed.
    pub fn merge_classes(&mut self, a: EClassId, b: EClassId) {
        if a == b {
            return;
        }
        let (keep, drop) = if a < b { (a, b) } else { (b, a) };
        let to_move: Vec<_> = self.eclass_to_causal.remove(&drop).unwrap_or_default();
        for c in &to_move {
            self.causal_to_eclass.insert(*c, keep);
        }
        self.eclass_to_causal
            .entry(keep)
            .or_default()
            .extend(to_move);
    }

    pub fn class_count(&self) -> usize {
        self.eclass_to_causal.len()
    }

    pub fn event_count(&self) -> usize {
        self.causal_to_eclass.len()
    }
}

#[cfg(feature = "rhizome")]
pub mod loro_integration {
    //! Wired in v0.2 once Loro 1.0 stabilizes.
    //! This module is intentionally empty in v0.1.0.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alloc_yields_sequential_ids() {
        let mut b = RhizomeBridge::new();
        assert_eq!(b.alloc_eclass(), 0);
        assert_eq!(b.alloc_eclass(), 1);
        assert_eq!(b.alloc_eclass(), 2);
    }

    #[test]
    fn insert_then_lookup_round_trip() {
        let mut b = RhizomeBridge::new();
        let e = b.alloc_eclass();
        b.insert(7, e);
        assert_eq!(b.eclass_of(7), Some(e));
        assert_eq!(b.causal_events_of(e), &[7u64][..]);
    }

    #[test]
    fn reinsert_under_new_class_updates_both_directions() {
        let mut b = RhizomeBridge::new();
        let e0 = b.alloc_eclass();
        let e1 = b.alloc_eclass();
        b.insert(7, e0);
        b.insert(7, e1);
        assert_eq!(b.eclass_of(7), Some(e1));
        assert!(b.causal_events_of(e0).is_empty());
        assert_eq!(b.causal_events_of(e1), &[7u64][..]);
    }

    #[test]
    fn merge_preserves_all_causal_ids() {
        let mut b = RhizomeBridge::new();
        let e0 = b.alloc_eclass();
        let e1 = b.alloc_eclass();
        b.insert(10, e0);
        b.insert(11, e0);
        b.insert(20, e1);
        b.merge_classes(e0, e1);
        assert_eq!(b.event_count(), 3);
        assert_eq!(b.class_count(), 1);
    }

    #[test]
    fn merge_is_no_op_for_self() {
        let mut b = RhizomeBridge::new();
        let e = b.alloc_eclass();
        b.insert(1, e);
        b.merge_classes(e, e);
        assert_eq!(b.event_count(), 1);
        assert_eq!(b.class_count(), 1);
    }

    #[test]
    fn class_of_unknown_causal_is_none() {
        let b = RhizomeBridge::new();
        assert_eq!(b.eclass_of(99), None);
    }
}

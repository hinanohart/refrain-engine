//! Rhizome bridge property tests.

use proptest::prelude::*;
use refrain_rhizome::RhizomeBridge;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]

    /// Alloc returns sequential, monotonically increasing ids.
    #[test]
    fn alloc_is_monotonic(n in 1usize..32) {
        let mut b = RhizomeBridge::new();
        let mut last: Option<u64> = None;
        for _ in 0..n {
            let id = b.alloc_eclass();
            if let Some(prev) = last {
                prop_assert!(id > prev);
            }
            last = Some(id);
        }
    }

    /// After inserting a causal id into class e, lookup returns e.
    #[test]
    fn insert_then_lookup_consistent(causals in prop::collection::vec(0u64..1024, 1..32)) {
        let mut b = RhizomeBridge::new();
        let e = b.alloc_eclass();
        for c in &causals {
            b.insert(*c, e);
        }
        for c in &causals {
            prop_assert_eq!(b.eclass_of(*c), Some(e));
        }
    }

    /// Merge is idempotent: merging twice gives the same state as once.
    #[test]
    fn merge_is_idempotent(
        c1 in 0u64..512,
        c2 in 512u64..1024,
    ) {
        let mut b1 = RhizomeBridge::new();
        let e_a = b1.alloc_eclass();
        let e_b = b1.alloc_eclass();
        b1.insert(c1, e_a);
        b1.insert(c2, e_b);
        b1.merge_classes(e_a, e_b);
        let after_once_a = b1.eclass_of(c1);
        let after_once_b = b1.eclass_of(c2);
        b1.merge_classes(e_a, e_b);
        prop_assert_eq!(b1.eclass_of(c1), after_once_a);
        prop_assert_eq!(b1.eclass_of(c2), after_once_b);
    }

    /// Class count never increases under merge.
    #[test]
    fn merge_does_not_grow_class_count(
        c1 in 0u64..512,
        c2 in 512u64..1024,
    ) {
        let mut b = RhizomeBridge::new();
        let e_a = b.alloc_eclass();
        let e_b = b.alloc_eclass();
        b.insert(c1, e_a);
        b.insert(c2, e_b);
        let before = b.class_count();
        b.merge_classes(e_a, e_b);
        prop_assert!(b.class_count() <= before);
    }
}

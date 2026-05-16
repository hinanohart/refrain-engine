//! Phase 4 gate: 100-case property test for Egraph::normalize.
//!
//! Properties checked:
//!   1. `normalize` does not panic on any structurally valid Refrain.
//!   2. `normalize` is idempotent: `normalize(normalize(r)) == normalize(r)`.
//!   3. Round-trip via JSON serialization preserves the result.

use proptest::prelude::*;
use refrain_core::{Op, Pattern, Refrain};
use refrain_egraph::Egraph;

fn arb_atom() -> impl Strategy<Value = String> {
    "[A-Za-z][A-Za-z0-9_-]{0,8}".prop_map(|s| s.to_string())
}

fn arb_pitch() -> impl Strategy<Value = String> {
    "[A-G](#|b)?[0-9]".prop_map(|s| s.to_string())
}

fn arb_dur() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("w".to_string()),
        Just("h".to_string()),
        Just("q".to_string()),
        Just("e".to_string()),
        Just("s".to_string()),
        Just("whole".to_string()),
    ]
}

fn arb_op_leaf() -> impl Strategy<Value = Op> {
    prop_oneof![
        (arb_pitch(), arb_dur()).prop_map(|(pitch, dur)| Op::Note { pitch, dur }),
        (arb_atom(), arb_atom()).prop_map(|(x, t)| Op::Diff { x, t }),
        prop::collection::vec(arb_atom(), 1..4).prop_map(|rels| Op::Quotient { rels }),
        arb_atom().prop_map(Op::Sym),
    ]
}

fn arb_pattern() -> impl Strategy<Value = Pattern> {
    let leaf = arb_op_leaf().prop_map(Pattern::Op);
    leaf.prop_recursive(3, 16, 4, |inner| {
        prop_oneof![
            // Loop with random count 0..5
            (0u32..5, inner.clone()).prop_map(|(count, body)| Pattern::Op(Op::Loop {
                count,
                body: Box::new(body),
            })),
            // Seq of 2..4 patterns
            prop::collection::vec(inner.clone(), 2..4).prop_map(Pattern::Seq),
            // Call with random head and 0..3 args
            (arb_atom(), prop::collection::vec(inner, 0..3))
                .prop_map(|(head, args)| Pattern::Op(Op::Call { head, args })),
        ]
    })
}

fn arb_refrain() -> impl Strategy<Value = Refrain> {
    (
        arb_atom(),
        prop::option::of(arb_pattern()),
        prop::option::of(arb_pattern()),
        prop::option::of(arb_pattern()),
    )
        .prop_map(|(name, t, d, r)| {
            let mut rf = Refrain::new(name);
            rf.territorialize = t;
            rf.deterritorialize = d;
            rf.reterritorialize = r;
            rf
        })
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn normalize_does_not_panic(r in arb_refrain()) {
        let e = Egraph::default();
        let _ = e.normalize(&r);
    }

    #[test]
    fn normalize_idempotent(r in arb_refrain()) {
        let e = Egraph::default();
        let n1 = e.normalize(&r).expect("normalize ok");
        let n2 = e.normalize(&n1).expect("normalize ok");
        prop_assert_eq!(n1, n2);
    }

    #[test]
    fn normalize_preserves_name(r in arb_refrain()) {
        let e = Egraph::default();
        let n = e.normalize(&r).expect("normalize ok");
        prop_assert_eq!(n.name, r.name);
    }

    #[test]
    fn normalize_preserves_stage_presence(r in arb_refrain()) {
        let e = Egraph::default();
        let n = e.normalize(&r).expect("normalize ok");
        prop_assert_eq!(n.territorialize.is_some(), r.territorialize.is_some());
        prop_assert_eq!(n.deterritorialize.is_some(), r.deterritorialize.is_some());
        prop_assert_eq!(n.reterritorialize.is_some(), r.reterritorialize.is_some());
    }

    #[test]
    fn json_roundtrip_after_normalize(r in arb_refrain()) {
        let e = Egraph::default();
        let n = e.normalize(&r).expect("normalize ok");
        let json = serde_json::to_string(&n).expect("serialize ok");
        let n2: Refrain = serde_json::from_str(&json).expect("deserialize ok");
        prop_assert_eq!(n, n2);
    }
}

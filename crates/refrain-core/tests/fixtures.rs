//! 20-fixture parse acceptance test for the Refrain DSL.

use refrain_core::{parse, Op, Pattern, Refrain};

const FIXTURES: &[(&str, &str)] = &[
    ("empty", "(refrain a)"),
    (
        "simple-note",
        "(refrain a (territorialize (note C4 q)))",
    ),
    (
        "three-stages",
        "(refrain a \
         (territorialize (note C4 q)) \
         (deterritorialize (note D4 q)) \
         (reterritorialize (note E4 q)))",
    ),
    (
        "loop",
        "(refrain a (territorialize (loop 4 (note C4 q))))",
    ),
    (
        "diff",
        "(refrain a (deterritorialize (dy/dx intensity time)))",
    ),
    (
        "quotient",
        "(refrain a (reterritorialize (quotient ~rotation ~transpose)))",
    ),
    (
        "nested-loop",
        "(refrain a (territorialize (loop 2 (loop 3 (note C4 e)))))",
    ),
    (
        "seq-notes",
        "(refrain a (territorialize (note C4 q) (note D4 q) (note E4 q)))",
    ),
    (
        "comment",
        "; comment line\n(refrain a (territorialize (note C4 q)))",
    ),
    (
        "whitespace",
        "(refrain\n  whitespace-name\n  (territorialize\n    (note C4 q)))",
    ),
    (
        "sharp-pitch",
        "(refrain a (territorialize (note G#3 e)))",
    ),
    (
        "long-dur",
        "(refrain a (territorialize (note C4 whole)))",
    ),
    (
        "complex",
        "(refrain melody \
         (territorialize (loop 4 (note G4 e))) \
         (deterritorialize (dy/dx pitch time)) \
         (reterritorialize (quotient ~octave)))",
    ),
    (
        "named-dash",
        "(refrain melody-a (territorialize (note C4 q)))",
    ),
    (
        "named-underscore",
        "(refrain melody_a (territorialize (note C4 q)))",
    ),
    (
        "many-quotient",
        "(refrain a (reterritorialize (quotient ~a ~b ~c ~d)))",
    ),
    (
        "deter-only",
        "(refrain a (deterritorialize (dy/dx x t)))",
    ),
    (
        "reter-only",
        "(refrain a (reterritorialize (quotient ~r)))",
    ),
    (
        "call-form",
        "(refrain a (territorialize (rest h)))",
    ),
    (
        "deep-nested",
        "(refrain a (territorialize (loop 2 (loop 3 (loop 4 (note C4 s))))))",
    ),
];

#[test]
fn all_twenty_fixtures_parse_successfully() {
    for (name, src) in FIXTURES {
        let r = parse(src);
        assert!(
            r.is_ok(),
            "fixture `{}` failed to parse: {:?}\nsource:\n{}",
            name,
            r.err(),
            src
        );
    }
    assert_eq!(FIXTURES.len(), 20, "expected exactly 20 fixtures");
}

#[test]
fn parse_extracts_name_from_simple_note() {
    let r = parse("(refrain melody-a (territorialize (note C4 q)))").unwrap();
    assert_eq!(r.name, "melody-a");
    match r.territorialize.as_ref().unwrap() {
        Pattern::Op(Op::Note { pitch, dur }) => {
            assert_eq!(pitch, "C4");
            assert_eq!(dur, "q");
        }
        other => panic!("expected Note, got {:?}", other),
    }
}

#[test]
fn parse_loop_count_is_u32() {
    let r = parse("(refrain a (territorialize (loop 4 (note C4 q))))").unwrap();
    match r.territorialize.as_ref().unwrap() {
        Pattern::Op(Op::Loop { count, body }) => {
            assert_eq!(*count, 4);
            assert!(matches!(**body, Pattern::Op(Op::Note { .. })));
        }
        other => panic!("expected Loop, got {:?}", other),
    }
}

#[test]
fn parse_diff_xt() {
    let r = parse("(refrain a (deterritorialize (dy/dx intensity time)))").unwrap();
    match r.deterritorialize.as_ref().unwrap() {
        Pattern::Op(Op::Diff { x, t }) => {
            assert_eq!(x, "intensity");
            assert_eq!(t, "time");
        }
        other => panic!("expected Diff, got {:?}", other),
    }
}

#[test]
fn parse_quotient_collects_rels() {
    let r = parse("(refrain a (reterritorialize (quotient ~a ~b ~c)))").unwrap();
    match r.reterritorialize.as_ref().unwrap() {
        Pattern::Op(Op::Quotient { rels }) => {
            assert_eq!(rels, &vec!["~a".to_string(), "~b".to_string(), "~c".to_string()]);
        }
        other => panic!("expected Quotient, got {:?}", other),
    }
}

#[test]
fn parse_sequence_becomes_seq() {
    let r = parse("(refrain a (territorialize (note C4 q) (note D4 q)))").unwrap();
    match r.territorialize.as_ref().unwrap() {
        Pattern::Seq(xs) => assert_eq!(xs.len(), 2),
        other => panic!("expected Seq, got {:?}", other),
    }
}

#[test]
fn parse_call_for_unknown_op() {
    let r = parse("(refrain a (territorialize (rest h)))").unwrap();
    match r.territorialize.as_ref().unwrap() {
        Pattern::Op(Op::Call { head, args }) => {
            assert_eq!(head, "rest");
            assert_eq!(args.len(), 1);
        }
        other => panic!("expected Call, got {:?}", other),
    }
}

#[test]
fn parse_rejects_unclosed_list() {
    let r = parse("(refrain a (territorialize (note C4 q)");
    assert!(r.is_err());
}

#[test]
fn parse_rejects_stray_close() {
    let r = parse(")");
    assert!(r.is_err());
}

#[test]
fn parse_rejects_bad_loop_count() {
    let r = parse("(refrain a (territorialize (loop xx (note C4 q))))");
    assert!(r.is_err());
}

#[test]
fn parse_rejects_unknown_stage_kind() {
    let r = parse("(refrain a (deterrorialize-typo (note C4 q)))");
    assert!(r.is_err());
}

#[test]
fn parse_rejects_missing_name() {
    let r = parse("(refrain)");
    assert!(r.is_err());
}

#[test]
fn parse_rejects_non_refrain_head() {
    let r = parse("(foo a)");
    assert!(r.is_err());
}

#[test]
fn parse_handles_comments() {
    let r = parse("; head comment\n(refrain a ; inline\n  (territorialize (note C4 q)))").unwrap();
    assert_eq!(r.name, "a");
}

#[test]
fn refrain_stages_iter_yields_only_present() {
    let r = parse("(refrain a (territorialize (note C4 q)) (reterritorialize (quotient ~r)))")
        .unwrap();
    let stages: Vec<_> = r.stages().collect();
    assert_eq!(stages.len(), 2);
}

#[test]
fn ast_serde_full_roundtrip() {
    let src = "(refrain melody \
              (territorialize (loop 4 (note G4 e))) \
              (deterritorialize (dy/dx pitch time)) \
              (reterritorialize (quotient ~octave)))";
    let r1: Refrain = parse(src).unwrap();
    let json = serde_json::to_string(&r1).unwrap();
    let r2: Refrain = serde_json::from_str(&json).unwrap();
    assert_eq!(r1, r2);
}

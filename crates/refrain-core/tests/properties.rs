//! Parser property tests: never panic, error on garbage, accept all valid
//! S-expression refrains, comments are transparent.

use proptest::prelude::*;
use refrain_core::parse;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]

    /// Parser never panics on any UTF-8 input. It either returns Ok or Err.
    #[test]
    fn parser_never_panics_on_arbitrary_text(s in "\\PC{0,200}") {
        let _ = parse(&s);
    }

    /// Adding/stripping a leading line comment is transparent.
    #[test]
    fn comment_prefix_is_transparent(
        name in "[a-z][a-z0-9_-]{0,8}",
        comment in "[^\\n]{0,50}",
    ) {
        let plain = format!("(refrain {})", name);
        let with_comment = format!("; {}\n(refrain {})", comment, name);
        let r1 = parse(&plain);
        let r2 = parse(&with_comment);
        prop_assert_eq!(r1.is_ok(), r2.is_ok());
        if let (Ok(a), Ok(b)) = (r1, r2) {
            prop_assert_eq!(a, b);
        }
    }

    /// All structurally valid simple refrains parse and round-trip the name.
    #[test]
    fn valid_refrains_round_trip_name(name in "[a-z][a-z0-9_-]{0,8}") {
        let src = format!("(refrain {})", name);
        let r = parse(&src).expect("valid refrain");
        prop_assert_eq!(r.name, name);
    }

    /// `(refrain N (territorialize (note P D)))` is always parseable.
    #[test]
    fn note_refrain_always_parses(
        name in "[a-z][a-z0-9_-]{0,6}",
        pitch in "[A-G](#|b)?[0-9]",
        dur in "[whqes]",
    ) {
        let src = format!("(refrain {} (territorialize (note {} {})))", name, pitch, dur);
        prop_assert!(parse(&src).is_ok());
    }
}

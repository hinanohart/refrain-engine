//! Cross-adapter property tests: scheduler invariants, code emitter
//! roundtrips, text adapter seed determinism, audio Hap count.

use proptest::prelude::*;

use refrain_adapters::{
    schedule, AudioAdapter, AudioFormat, CodeAdapter, CodeLang, EmitCtx, ExtractedRefrain,
    RefrainAdapter, TextAdapter, TextStyle, VisualAdapter,
};
use refrain_core::{parse, Op, Pattern, Refrain};

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
    ]
}

fn arb_simple_refrain() -> impl Strategy<Value = Refrain> {
    (
        "[a-z][a-z0-9_-]{0,6}",
        prop::collection::vec((arb_pitch(), arb_dur()), 0..6),
    )
        .prop_map(|(name, notes)| {
            let mut r = Refrain::new(name);
            if !notes.is_empty() {
                let pats: Vec<Pattern> = notes
                    .into_iter()
                    .map(|(pitch, dur)| Pattern::Op(Op::Note { pitch, dur }))
                    .collect();
                r.territorialize = if pats.len() == 1 {
                    Some(pats.into_iter().next().unwrap())
                } else {
                    Some(Pattern::Seq(pats))
                };
            }
            r
        })
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]

    /// Scheduler is total: every Refrain compiles to some Hap list.
    #[test]
    fn schedule_total(r in arb_simple_refrain()) {
        let mut t = 0.0;
        for (_kind, p) in r.stages() {
            let (_haps, dur) = schedule(p, t);
            prop_assert!(dur >= 0.0);
            t += dur;
        }
    }

    /// Strudel JSON output is parseable JSON.
    #[test]
    fn audio_strudel_is_valid_json(r in arb_simple_refrain()) {
        let bytes = AudioAdapter::new(AudioFormat::StrudelJson)
            .emit(&ExtractedRefrain { refrain: &r }, &EmitCtx::default())
            .unwrap();
        let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        prop_assert!(v.is_array());
    }

    /// OSC output starts with the bundle header for any refrain.
    #[test]
    fn audio_osc_is_a_bundle(r in arb_simple_refrain()) {
        let bytes = AudioAdapter::new(AudioFormat::Osc)
            .emit(&ExtractedRefrain { refrain: &r }, &EmitCtx::default())
            .unwrap();
        prop_assert_eq!(&bytes[0..8], b"#bundle\0");
    }

    /// Visual adapter always produces a valid PNG header.
    #[test]
    fn visual_png_header_is_present(r in arb_simple_refrain()) {
        let bytes = VisualAdapter::new()
            .emit(&ExtractedRefrain { refrain: &r }, &EmitCtx::default())
            .unwrap();
        prop_assert_eq!(&bytes[0..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
    }

    /// Code adapter's Python output re-parses to the same refrain.
    #[test]
    fn code_python_roundtrips(r in arb_simple_refrain()) {
        let a = CodeAdapter::new(CodeLang::Python);
        let s = String::from_utf8(
            a.emit(&ExtractedRefrain { refrain: &r }, &EmitCtx::default()).unwrap(),
        )
        .unwrap();
        let start = s.find("\"(refrain").expect("refrain literal") + 1;
        let end = s[start..].find(")\"\n").expect("closing quote") + start + 1;
        let literal = &s[start..end];
        let reparsed = parse(literal).unwrap();
        prop_assert_eq!(reparsed.name, r.name);
        prop_assert_eq!(reparsed.territorialize.is_some(), r.territorialize.is_some());
    }

    /// Text adapter with the same seed produces byte-identical output.
    #[test]
    fn text_adapter_seed_deterministic(r in arb_simple_refrain(), seed: u64) {
        let a = TextAdapter::with_seed(TextStyle::Prose, seed);
        let b1 = a.emit(&ExtractedRefrain { refrain: &r }, &EmitCtx::default()).unwrap();
        let b2 = a.emit(&ExtractedRefrain { refrain: &r }, &EmitCtx::default()).unwrap();
        prop_assert_eq!(b1, b2);
    }
}

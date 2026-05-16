//! Workspace-level integration smoke test:
//! parse → normalize → emit through each adapter.

use refrain_adapters::{
    AudioAdapter, AudioFormat, CodeAdapter, CodeLang, EmitCtx, ExtractedRefrain, RefrainAdapter,
    TextAdapter, TextStyle, VisualAdapter,
};
use refrain_core::parse;
use refrain_egraph::Egraph;

fn canonical_refrain() -> &'static str {
    "(refrain melody-a \
     (territorialize (loop 4 (note C4 q))) \
     (deterritorialize (dy/dx intensity time)) \
     (reterritorialize (quotient ~rotation ~transpose)))"
}

#[test]
fn parse_normalize_emit_audio_strudel() {
    let r = parse(canonical_refrain()).unwrap();
    let n = Egraph::default().normalize(&r).unwrap();
    let bytes = AudioAdapter::new(AudioFormat::StrudelJson)
        .emit(&ExtractedRefrain { refrain: &n }, &EmitCtx::default())
        .unwrap();
    assert!(std::str::from_utf8(&bytes).unwrap().contains("C4"));
}

#[test]
fn parse_normalize_emit_audio_osc() {
    let r = parse(canonical_refrain()).unwrap();
    let n = Egraph::default().normalize(&r).unwrap();
    let bytes = AudioAdapter::new(AudioFormat::Osc)
        .emit(&ExtractedRefrain { refrain: &n }, &EmitCtx::default())
        .unwrap();
    assert_eq!(&bytes[0..8], b"#bundle\0");
}

#[test]
fn parse_normalize_emit_visual_png() {
    let r = parse(canonical_refrain()).unwrap();
    let n = Egraph::default().normalize(&r).unwrap();
    let bytes = VisualAdapter::new()
        .emit(&ExtractedRefrain { refrain: &n }, &EmitCtx::default())
        .unwrap();
    assert_eq!(&bytes[0..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
}

#[test]
fn parse_normalize_emit_code_python() {
    let r = parse(canonical_refrain()).unwrap();
    let n = Egraph::default().normalize(&r).unwrap();
    let bytes = CodeAdapter::new(CodeLang::Python)
        .emit(&ExtractedRefrain { refrain: &n }, &EmitCtx::default())
        .unwrap();
    let s = std::str::from_utf8(&bytes).unwrap();
    assert!(s.contains("from refrain_py import _native"));
    assert!(s.contains("(refrain melody-a"));
}

#[test]
fn parse_normalize_emit_text_prose_deterministic() {
    let r = parse(canonical_refrain()).unwrap();
    let n = Egraph::default().normalize(&r).unwrap();
    let a = TextAdapter::with_seed(TextStyle::Prose, 1);
    let b1 = a
        .emit(&ExtractedRefrain { refrain: &n }, &EmitCtx::default())
        .unwrap();
    let b2 = a
        .emit(&ExtractedRefrain { refrain: &n }, &EmitCtx::default())
        .unwrap();
    assert_eq!(b1, b2);
    assert!(std::str::from_utf8(&b1)
        .unwrap()
        .contains("# Refrain: melody-a"));
}

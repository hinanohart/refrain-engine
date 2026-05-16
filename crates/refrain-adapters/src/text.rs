//! Text adapter: deterministic natural-language template rendering.
//!
//! Each Hap is rendered as one line of "human-readable" prose. The
//! template is intentionally minimal — a heavier markov or n-gram
//! generator would need a corpus and would not be byte-deterministic
//! across machines. For Phase 9 we ship a stable template renderer and
//! a tiny seeded shuffler that produces stylistic variation on the
//! filler words while keeping the structural slots fixed.

use refrain_core::Refrain;

use crate::schedule::{schedule, Hap};
use crate::{AdapterCaps, AdapterErr, EmitCtx, ExtractedRefrain, RefrainAdapter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextStyle {
    Prose,
    Bullets,
}

pub struct TextAdapter {
    pub style: TextStyle,
    pub seed: u64,
}

impl TextAdapter {
    pub fn new(style: TextStyle) -> Self {
        Self { style, seed: 0 }
    }

    pub fn with_seed(style: TextStyle, seed: u64) -> Self {
        Self { style, seed }
    }
}

fn collect_haps(refrain: &Refrain) -> Vec<Hap> {
    let mut all = Vec::new();
    let mut t = 0.0;
    for (_kind, p) in refrain.stages() {
        let (sub, dur) = schedule(p, t);
        all.extend(sub);
        t += dur;
    }
    all
}

// A small seeded LCG so the same (refrain, seed) yields the same output.
fn lcg(state: &mut u64) -> u64 {
    *state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    *state
}

const VERBS: &[&str] = &["sings", "chants", "intones", "voices", "speaks"];
const SUFFIXES: &[&str] = &[".", " in measured time.", " over the refrain.", " quietly."];

fn render_hap(h: &Hap, state: &mut u64, style: TextStyle) -> String {
    let verb = VERBS[(lcg(state) as usize) % VERBS.len()];
    let suffix = SUFFIXES[(lcg(state) as usize) % SUFFIXES.len()];
    match style {
        TextStyle::Prose => match &h.pitch {
            Some(p) => format!(
                "At cycle {:.4}, the voice {} {} for {:.4} cycles{}",
                h.start, verb, p, h.duration(), suffix
            ),
            None => format!("At cycle {:.4}, a structural mark: {}{}", h.start, h.value, suffix),
        },
        TextStyle::Bullets => match &h.pitch {
            Some(p) => format!("- t={:.4} {} {} dur={:.4}", h.start, verb, p, h.duration()),
            None => format!("- t={:.4} mark={}", h.start, h.value),
        },
    }
}

impl RefrainAdapter for TextAdapter {
    fn name(&self) -> &str {
        match self.style {
            TextStyle::Prose => "text.prose",
            TextStyle::Bullets => "text.bullets",
        }
    }

    fn emit(&self, refrain: &ExtractedRefrain, _ctx: &EmitCtx) -> Result<Vec<u8>, AdapterErr> {
        let haps = collect_haps(refrain.refrain);
        let mut state = self.seed.wrapping_add(0x9E3779B97F4A7C15); // stable salt
        let mut out = String::new();
        out.push_str("# Refrain: ");
        out.push_str(&refrain.refrain.name);
        out.push('\n');
        for h in &haps {
            out.push_str(&render_hap(h, &mut state, self.style));
            out.push('\n');
        }
        Ok(out.into_bytes())
    }

    fn capabilities(&self) -> AdapterCaps {
        AdapterCaps {
            realtime: false,
            differentiable: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use refrain_core::parse;

    #[test]
    fn prose_contains_pitch_lines() {
        let r = parse("(refrain a (territorialize (loop 4 (note C4 q))))").unwrap();
        let a = TextAdapter::new(TextStyle::Prose);
        let s = String::from_utf8(a.emit(&ExtractedRefrain { refrain: &r }, &EmitCtx::default()).unwrap()).unwrap();
        assert!(s.contains("# Refrain: a"));
        assert_eq!(s.matches("C4").count(), 4);
    }

    #[test]
    fn bullets_uses_dash_prefix() {
        let r = parse("(refrain b (territorialize (note G4 e)))").unwrap();
        let a = TextAdapter::new(TextStyle::Bullets);
        let s = String::from_utf8(a.emit(&ExtractedRefrain { refrain: &r }, &EmitCtx::default()).unwrap()).unwrap();
        assert!(s.contains("- t="));
        assert!(s.contains("G4"));
    }

    #[test]
    fn deterministic_for_same_seed() {
        let r = parse("(refrain c (territorialize (loop 4 (note C4 q))))").unwrap();
        let a = TextAdapter::with_seed(TextStyle::Prose, 42);
        let s1 = a.emit(&ExtractedRefrain { refrain: &r }, &EmitCtx::default()).unwrap();
        let s2 = a.emit(&ExtractedRefrain { refrain: &r }, &EmitCtx::default()).unwrap();
        assert_eq!(s1, s2);
    }

    #[test]
    fn different_seeds_diverge() {
        let r = parse("(refrain c (territorialize (loop 4 (note C4 q))))").unwrap();
        let a = TextAdapter::with_seed(TextStyle::Prose, 1);
        let b = TextAdapter::with_seed(TextStyle::Prose, 2);
        let sa = a.emit(&ExtractedRefrain { refrain: &r }, &EmitCtx::default()).unwrap();
        let sb = b.emit(&ExtractedRefrain { refrain: &r }, &EmitCtx::default()).unwrap();
        assert_ne!(sa, sb);
    }

    #[test]
    fn empty_refrain_emits_just_header() {
        let r = parse("(refrain e)").unwrap();
        let a = TextAdapter::new(TextStyle::Prose);
        let s = String::from_utf8(a.emit(&ExtractedRefrain { refrain: &r }, &EmitCtx::default()).unwrap()).unwrap();
        assert_eq!(s, "# Refrain: e\n");
    }

    #[test]
    fn names_distinguish_styles() {
        assert_eq!(TextAdapter::new(TextStyle::Prose).name(), "text.prose");
        assert_eq!(TextAdapter::new(TextStyle::Bullets).name(), "text.bullets");
    }
}

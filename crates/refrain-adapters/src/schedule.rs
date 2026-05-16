//! Pattern → time-event schedule.
//!
//! Compiles a `Pattern` into a flat list of `Hap` (happening) events with
//! absolute time stamps in cycles. Adapters consume the schedule and
//! translate it to media-specific output (audio Hap JSON, OSC packets,
//! visual draw calls, text streams, etc.).

use refrain_core::{Op, Pattern};

/// One scheduled happening: a value with a half-open time interval [start, end).
#[derive(Debug, Clone, PartialEq)]
pub struct Hap {
    /// Cycle-relative start time.
    pub start: f64,
    /// Cycle-relative end time (start + duration).
    pub end: f64,
    /// Pitch name (e.g. "C4") or atom string.
    pub pitch: Option<String>,
    /// Free-form payload (kept stringly typed for v0.1; structured value in v0.2).
    pub value: String,
}

impl Hap {
    pub fn duration(&self) -> f64 {
        self.end - self.start
    }
}

/// Standard musical-symbol → cycle-fraction map.
fn dur_to_cycles(d: &str) -> f64 {
    match d {
        "w" | "whole" => 1.0,
        "h" | "half" => 0.5,
        "q" | "quarter" => 0.25,
        "e" | "eighth" => 0.125,
        "s" | "sixteenth" => 0.0625,
        // Numeric duration in cycles (e.g. "0.5") falls through to parse.
        s => s.parse::<f64>().unwrap_or(0.25),
    }
}

/// Compile a pattern starting at `t0`. Returns the list of Haps and the
/// total duration consumed.
pub fn schedule(pattern: &Pattern, t0: f64) -> (Vec<Hap>, f64) {
    match pattern {
        Pattern::Op(op) => schedule_op(op, t0),
        Pattern::Seq(items) => {
            let mut t = t0;
            let mut out = Vec::with_capacity(items.len());
            for p in items {
                let (sub, dur) = schedule(p, t);
                out.extend(sub);
                t += dur;
            }
            (out, t - t0)
        }
    }
}

fn schedule_op(op: &Op, t0: f64) -> (Vec<Hap>, f64) {
    match op {
        Op::Note { pitch, dur } => {
            let d = dur_to_cycles(dur);
            (
                vec![Hap {
                    start: t0,
                    end: t0 + d,
                    pitch: Some(pitch.clone()),
                    value: format!("note:{}:{}", pitch, dur),
                }],
                d,
            )
        }
        Op::Loop { count, body } => {
            let mut t = t0;
            let mut out = Vec::new();
            for _ in 0..*count {
                let (sub, dur) = schedule(body, t);
                out.extend(sub);
                t += dur;
            }
            (out, t - t0)
        }
        Op::Diff { x, t } => (
            vec![Hap {
                start: t0,
                end: t0,
                pitch: None,
                value: format!("diff:{}:{}", x, t),
            }],
            0.0,
        ),
        Op::Quotient { rels } => (
            vec![Hap {
                start: t0,
                end: t0,
                pitch: None,
                value: format!("quotient:{}", rels.join(",")),
            }],
            0.0,
        ),
        Op::Sym(s) => (
            vec![Hap {
                start: t0,
                end: t0,
                pitch: None,
                value: format!("sym:{}", s),
            }],
            0.0,
        ),
        Op::Call { head, args } => {
            let mut out = Vec::new();
            let mut t = t0;
            for a in args {
                let (sub, dur) = schedule(a, t);
                out.extend(sub);
                t += dur;
            }
            out.push(Hap {
                start: t0,
                end: t,
                pitch: None,
                value: format!("call:{}", head),
            });
            (out, t - t0)
        }
    }
}

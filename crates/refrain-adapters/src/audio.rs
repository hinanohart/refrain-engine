//! Audio adapter: emits either Strudel-style Hap JSON or OSC bundle bytes.
//!
//! Strudel format reference: <https://strudel.tidalcycles.org/>
//! OSC format reference: <https://opensoundcontrol.stanford.edu/spec-1_0.html>

use rosc::{OscBundle, OscMessage, OscPacket, OscTime, OscType};
use serde::Serialize;

use refrain_core::Refrain;

use crate::schedule::{schedule, Hap};
use crate::{AdapterCaps, AdapterErr, EmitCtx, ExtractedRefrain, RefrainAdapter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioFormat {
    StrudelJson,
    Osc,
}

#[derive(Debug, Clone, Serialize)]
struct StrudelHap {
    whole: StrudelSpan,
    part: StrudelSpan,
    value: StrudelValue,
}

#[derive(Debug, Clone, Serialize)]
struct StrudelSpan {
    begin: f64,
    end: f64,
}

#[derive(Debug, Clone, Serialize)]
struct StrudelValue {
    note: Option<String>,
    raw: String,
}

pub struct AudioAdapter {
    pub format: AudioFormat,
}

impl AudioAdapter {
    pub fn new(format: AudioFormat) -> Self {
        Self { format }
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

    fn emit_strudel(&self, haps: &[Hap]) -> Result<Vec<u8>, AdapterErr> {
        let json_haps: Vec<StrudelHap> = haps
            .iter()
            .map(|h| StrudelHap {
                whole: StrudelSpan {
                    begin: h.start,
                    end: h.end,
                },
                part: StrudelSpan {
                    begin: h.start,
                    end: h.end,
                },
                value: StrudelValue {
                    note: h.pitch.clone(),
                    raw: h.value.clone(),
                },
            })
            .collect();
        serde_json::to_vec_pretty(&json_haps)
            .map_err(|e| AdapterErr::Encoding(format!("strudel json: {}", e)))
    }

    fn emit_osc(&self, haps: &[Hap]) -> Result<Vec<u8>, AdapterErr> {
        let mut messages: Vec<OscPacket> = Vec::with_capacity(haps.len());
        for h in haps {
            let mut args: Vec<OscType> = Vec::new();
            args.push(OscType::Float(h.start as f32));
            args.push(OscType::Float(h.duration() as f32));
            if let Some(p) = &h.pitch {
                args.push(OscType::String(p.clone()));
            } else {
                args.push(OscType::String(h.value.clone()));
            }
            messages.push(OscPacket::Message(OscMessage {
                addr: "/refrain/note".into(),
                args,
            }));
        }
        let bundle = OscBundle {
            timetag: OscTime {
                seconds: 0,
                fractional: 0,
            },
            content: messages,
        };
        rosc::encoder::encode(&OscPacket::Bundle(bundle))
            .map_err(|e| AdapterErr::Encoding(format!("osc: {}", e)))
    }
}

impl RefrainAdapter for AudioAdapter {
    fn name(&self) -> &str {
        match self.format {
            AudioFormat::StrudelJson => "audio.strudel-json",
            AudioFormat::Osc => "audio.osc",
        }
    }

    fn emit(&self, refrain: &ExtractedRefrain, _ctx: &EmitCtx) -> Result<Vec<u8>, AdapterErr> {
        let haps = Self::collect_haps(refrain.refrain);
        match self.format {
            AudioFormat::StrudelJson => self.emit_strudel(&haps),
            AudioFormat::Osc => self.emit_osc(&haps),
        }
    }

    fn capabilities(&self) -> AdapterCaps {
        AdapterCaps {
            realtime: matches!(self.format, AudioFormat::Osc),
            differentiable: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use refrain_core::parse;

    #[test]
    fn strudel_emits_valid_json() {
        let r = parse("(refrain a (territorialize (loop 4 (note C4 q))))").unwrap();
        let a = AudioAdapter::new(AudioFormat::StrudelJson);
        let ex = ExtractedRefrain { refrain: &r };
        let bytes = a.emit(&ex, &EmitCtx::default()).unwrap();
        let s = std::str::from_utf8(&bytes).unwrap();
        assert!(s.contains("\"note\""));
        assert!(s.contains("\"C4\""));
        let parsed: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        let arr = parsed.as_array().unwrap();
        assert_eq!(arr.len(), 4);
    }

    #[test]
    fn osc_emits_non_empty_bundle() {
        let r = parse("(refrain a (territorialize (loop 4 (note C4 q))))").unwrap();
        let a = AudioAdapter::new(AudioFormat::Osc);
        let ex = ExtractedRefrain { refrain: &r };
        let bytes = a.emit(&ex, &EmitCtx::default()).unwrap();
        assert!(!bytes.is_empty());
        // OSC bundles begin with "#bundle\0".
        assert_eq!(&bytes[0..8], b"#bundle\0");
    }

    #[test]
    fn schedule_for_loop_four_quarters_spans_one_cycle() {
        let r = parse("(refrain a (territorialize (loop 4 (note C4 q))))").unwrap();
        let haps = AudioAdapter::collect_haps(&r);
        assert_eq!(haps.len(), 4);
        let total: f64 = haps.last().unwrap().end - haps.first().unwrap().start;
        assert_eq!(total, 1.0);
    }

    #[test]
    fn empty_refrain_yields_no_haps() {
        let r = parse("(refrain empty)").unwrap();
        let a = AudioAdapter::new(AudioFormat::StrudelJson);
        let ex = ExtractedRefrain { refrain: &r };
        let bytes = a.emit(&ex, &EmitCtx::default()).unwrap();
        let parsed: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(parsed.as_array().unwrap().len(), 0);
    }

    #[test]
    fn name_reflects_format() {
        assert_eq!(
            AudioAdapter::new(AudioFormat::StrudelJson).name(),
            "audio.strudel-json"
        );
        assert_eq!(AudioAdapter::new(AudioFormat::Osc).name(), "audio.osc");
    }
}

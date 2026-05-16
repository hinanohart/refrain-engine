//! refrain-adapters: pluggable output adapters for refrains.
//!
//! Defines the `RefrainAdapter` trait and provides built-in implementations
//! for audio (Strudel JSON / OSC), visual (wgpu/skia), code-rewrite (text
//! template), and text (n-gram). Built-ins are populated in Phases 6-9.

pub mod audio;
pub mod schedule;

pub use audio::{AudioAdapter, AudioFormat};
pub use schedule::{schedule, Hap};

use refrain_core::Refrain;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AdapterErr {
    #[error("adapter not implemented yet: {0}")]
    NotImplemented(String),
    #[error("encoding error: {0}")]
    Encoding(String),
}

#[derive(Debug, Clone, Default)]
pub struct AdapterCaps {
    pub realtime: bool,
    pub differentiable: bool,
}

#[derive(Debug, Clone, Default)]
pub struct EmitCtx {
    pub sample_rate: Option<u32>,
    pub frame_count: Option<u32>,
}

pub struct ExtractedRefrain<'a> {
    pub refrain: &'a Refrain,
}

pub trait RefrainAdapter: Send + Sync {
    fn name(&self) -> &str;
    fn emit(&self, refrain: &ExtractedRefrain, ctx: &EmitCtx) -> Result<Vec<u8>, AdapterErr>;
    fn capabilities(&self) -> AdapterCaps;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct DummyAdapter;
    impl RefrainAdapter for DummyAdapter {
        fn name(&self) -> &str {
            "dummy"
        }
        fn emit(&self, _r: &ExtractedRefrain, _ctx: &EmitCtx) -> Result<Vec<u8>, AdapterErr> {
            Ok(vec![])
        }
        fn capabilities(&self) -> AdapterCaps {
            AdapterCaps::default()
        }
    }

    #[test]
    fn dummy_adapter_emits_empty() {
        let r = Refrain::new("dummy-host");
        let ex = ExtractedRefrain { refrain: &r };
        let bytes = DummyAdapter.emit(&ex, &EmitCtx::default()).unwrap();
        assert!(bytes.is_empty());
        assert_eq!(DummyAdapter.name(), "dummy");
    }
}

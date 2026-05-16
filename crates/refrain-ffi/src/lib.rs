//! refrain-ffi: PyO3 + Arrow IPC bridge for the Python intensity_plane adapter.
//!
//! Phase 5 wires up `#[pyfunction]` exports and Arrow zero-copy IPC.

use refrain_core::Refrain;
use refrain_core::Result;

pub fn refrain_to_json(r: &Refrain) -> Result<String> {
    Ok(serde_json::to_string(r)?)
}

pub fn refrain_from_json(s: &str) -> Result<Refrain> {
    Ok(serde_json::from_str(s)?)
}

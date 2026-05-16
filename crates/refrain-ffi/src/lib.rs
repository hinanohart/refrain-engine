//! refrain-ffi: PyO3 bindings for `refrain_py._native`.
//!
//! Exposes the parser and the e-graph normalizer to Python. The boundary
//! is JSON for v0.1.0; Arrow IPC zero-copy lands when adapters need to
//! ship binary buffers (Phase 7+).

// The pyo3 0.22 #[pyfunction] / #[pymodule] macros expand to
// `Into::<PyErr>::into(...)` even when the value is already a `PyErr`.
// The lint fires on the macro output, not on our source — suppress at
// crate scope rather than touching macro-internal code.
#![allow(clippy::useless_conversion)]

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use refrain_core::Refrain;
use refrain_core::Result as RefrainResult;
use refrain_egraph::Egraph;

pub fn refrain_to_json(r: &Refrain) -> RefrainResult<String> {
    Ok(serde_json::to_string(r)?)
}

pub fn refrain_from_json(s: &str) -> RefrainResult<Refrain> {
    Ok(serde_json::from_str(s)?)
}

#[pyfunction]
fn parse_refrain(src: &str) -> PyResult<String> {
    let r = refrain_core::parse(src).map_err(|e| PyValueError::new_err(e.to_string()))?;
    serde_json::to_string(&r).map_err(|e| PyValueError::new_err(e.to_string()))
}

#[pyfunction]
fn normalize_refrain(json_src: &str) -> PyResult<String> {
    let r: Refrain = serde_json::from_str(json_src)
        .map_err(|e| PyValueError::new_err(format!("invalid refrain JSON: {}", e)))?;
    let e = Egraph::default();
    let n = e
        .normalize(&r)
        .map_err(|e| PyValueError::new_err(e.to_string()))?;
    serde_json::to_string(&n).map_err(|e| PyValueError::new_err(e.to_string()))
}

#[pyfunction]
fn parse_and_normalize(src: &str) -> PyResult<String> {
    let r = refrain_core::parse(src).map_err(|e| PyValueError::new_err(e.to_string()))?;
    let e = Egraph::default();
    let n = e
        .normalize(&r)
        .map_err(|e| PyValueError::new_err(e.to_string()))?;
    serde_json::to_string(&n).map_err(|e| PyValueError::new_err(e.to_string()))
}

#[pyfunction]
fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// The native module exposed as `refrain_py._native`.
#[pymodule]
fn _native(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_refrain, m)?)?;
    m.add_function(wrap_pyfunction!(normalize_refrain, m)?)?;
    m.add_function(wrap_pyfunction!(parse_and_normalize, m)?)?;
    m.add_function(wrap_pyfunction!(version, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use refrain_core::Refrain;

    #[test]
    fn json_roundtrip() {
        let r = Refrain::new("via-ffi");
        let s = refrain_to_json(&r).unwrap();
        let r2 = refrain_from_json(&s).unwrap();
        assert_eq!(r, r2);
    }
}

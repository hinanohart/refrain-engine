# Tests

Per-crate unit tests live under each `crates/*/src/` (inline `#[cfg(test)]`)
and under `crates/*/tests/` (integration tests). The five workspace-level
parse → normalize → emit smoke tests live at
[`crates/refrain-adapters/tests/integration.rs`](../crates/refrain-adapters/tests/integration.rs).

Python tests live at [`python/tests/`](../python/tests/).

```bash
cargo test --workspace
.venv/bin/pytest python/tests
```

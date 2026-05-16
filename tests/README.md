# Workspace integration tests

Per-crate unit tests live under each `crates/*/tests/`. This directory holds
**workspace-level integration tests** that cross crate boundaries — for
example, parse → normalize → emit smoke tests.

Populated in Phase 9 (after all four adapters land) and Phase 11 (docs/golden
fixture pass).

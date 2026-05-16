# Risks tracker

Living document of risks that could block v0.1.0 release.

## Loro `1.0.0-beta.5` pin

The `loro` crate is pinned to `1.0.0-beta.5` in `Cargo.toml`. Beta releases on
crates.io can be **yanked**.

- **Detection**: CI on every push runs `cargo deny` which reports yanked deps.
- **Mitigation**: rhizome support is behind `--features rhizome`; if Loro is
  yanked, the default-feature build keeps working.
- **Fallback**: if Loro freezes beta indefinitely we will (a) pin to a git
  revision in `Cargo.toml`, or (b) write a thin in-house event-graph CRDT
  shim sufficient for the two-layer HashMap bridge (Phase 10 scope).

## JAX on WSL2 (CPU-only)

Development runs on WSL2; CUDA-accelerated JAX is not available. The intensity
plane code path is forced to CPU JAX.

- **Mitigation**: the autodiff workload for v0.1.0 is small (`dyt` and
  `ehrhard_regnier_d` over patterns of < 100 nodes), so CPU is sufficient.
- **CI**: pinned to CPU JAX; the GitHub Actions `ubuntu-latest` runner is
  also CPU-only.

## `egg` vs `egglog` (terminology drift)

The architecture document mentioned `egglog` (Datalog frontend on top of e-graphs).
The implementation uses `egg` (the underlying e-graph crate) because egglog 0.4's
API churned during the Phase 4 window.

- **Status**: documented in `Cargo.toml` and `CHANGELOG.md`; egglog Datalog rules
  are scheduled for v0.2.
- **No user-visible promise broken** because v0.1.0 only ships the term-rewrite
  subset, which `egg` provides natively.

## Compact / context loss during autonomous build

Anthropic's Claude Code may compact the conversation at any time during this
multi-phase build. State is persisted via:

- `/home/runza/.claude/projects/-home-runza/memory/refrain_state.json` (the
  authoritative current-phase pointer).
- The git commit history on `phase/NN-*` branches.

On resume the worker reads `refrain_state.json`, finds the last completed phase,
checks out the next phase branch (or continues the in-progress one), and resumes.

## Bus factor 1

This is a single-maintainer project. The roadmap deliberately keeps the LOC
budget under 15k and the language count at two (Rust + Python) to keep it
tractable. Contributions are welcomed but not assumed.

# Examples

Standalone `.refrain` example files and per-adapter `cargo run` examples
are scheduled for **v0.2**.

For v0.1.0, the canonical end-to-end demonstration lives in
[`crates/refrain-adapters/tests/integration.rs`](../crates/refrain-adapters/tests/integration.rs).
It parses, normalizes, and emits the following refrain through every
built-in adapter:

```lisp
(refrain melody-a
  (territorialize (loop 4 (note C4 q)))
  (deterritorialize (dy/dx intensity time))
  (reterritorialize (quotient ~rotation ~transpose)))
```

Run it with:

```bash
cargo test -p refrain-adapters --test integration -- --nocapture
```

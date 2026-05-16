# Examples

Working refrains across the four adapters. This directory is populated
alongside Phases 6–9 (audio / visual / code / text):

| File | Phase | Adapter | Description |
|---|---|---|---|
| `melody_loop.refrain` | 6 | audio (Strudel/OSC) | 4-bar looped C major scale |
| `bouncing_circle.refrain` | 7 | visual (wgpu/skia) | Cell-complex over time |
| `code_rewrite_demo.refrain` | 8 | code (template) | Refrain → Python source |
| `markov_lyric.refrain` | 9 | text (n-gram) | Repeated lyric synthesis |

Each refrain file is a single S-expression. To run an example once the
adapters land:

```bash
cargo run --release --example refrain -- examples/melody_loop.refrain --adapter audio
```

The complete example walkthroughs live in
[`docs/examples.md`](../docs/examples.md) once written (Phase 11).

# Adapters

The `refrain-adapters` crate ships four built-in implementations of the
`RefrainAdapter` trait.

## Schedule

Every adapter consumes a `Vec<Hap>` produced by `schedule(&Pattern, t0)`.
A `Hap` is a (start, end, pitch, value) tuple in cycle time. Durations
follow the standard map: `w=1`, `h=0.5`, `q=0.25`, `e=0.125`, `s=0.0625`
cycles. Numeric literals fall through to `f64`.

## Audio

```rust
use refrain_adapters::{AudioAdapter, AudioFormat};

let a = AudioAdapter::new(AudioFormat::StrudelJson);
// or AudioFormat::Osc for OSC bundle bytes.
```

- **`AudioFormat::StrudelJson`** emits an array of Hap objects matching
  the [Strudel](https://strudel.tidalcycles.org/) format
  (`{ whole, part, value: { note, raw } }`). Pretty-printed JSON.
- **`AudioFormat::Osc`** emits an OSC bundle containing one
  `/refrain/note` message per Hap. Args: `start: f32`, `duration: f32`,
  `pitch_or_value: string`.

## Visual

```rust
use refrain_adapters::VisualAdapter;

let v = VisualAdapter::with_size(256, 256);
```

Renders a deterministic PNG. The canvas is split into three horizontal
bands (one per stage); each Hap renders as a colored rectangle whose
color is a stable hash of pitch/value mapped through a 24-color palette.
Output is byte-deterministic — encoding is pinned to PNG
`compression=Default, filter=NoFilter`.

## Code

```rust
use refrain_adapters::{CodeAdapter, CodeLang};

let py = CodeAdapter::new(CodeLang::Python);
let rs = CodeAdapter::new(CodeLang::Rust);
```

Emits source code that reconstructs the refrain. The Python output
calls `refrain_py._native.parse_refrain`; the Rust output calls
`refrain_core::parse`. Both round-trip through the parser to the same
AST.

## Text

```rust
use refrain_adapters::{TextAdapter, TextStyle};

let t = TextAdapter::with_seed(TextStyle::Prose, 42);
```

Two styles: `Prose` (one sentence per Hap) and `Bullets` (compact
dashed entries). A small LCG seeded by `seed + 0x9E3779B97F4A7C15`
produces stylistic variation on filler words while keeping structural
slots fixed — same `(refrain, seed)` always yields byte-identical
output.

## Writing a new adapter

```rust
use refrain_adapters::{
    AdapterCaps, AdapterErr, EmitCtx, ExtractedRefrain, RefrainAdapter,
};

pub struct MyAdapter;

impl RefrainAdapter for MyAdapter {
    fn name(&self) -> &str { "my-adapter" }
    fn emit(&self, refrain: &ExtractedRefrain, _ctx: &EmitCtx)
        -> Result<Vec<u8>, AdapterErr>
    {
        // Produce bytes from refrain.refrain.
        Ok(Vec::new())
    }
    fn capabilities(&self) -> AdapterCaps { AdapterCaps::default() }
}
```

Future versions will support `inventory::submit!`-style auto-registration;
v0.1 requires manual instantiation.

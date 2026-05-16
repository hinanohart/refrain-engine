# Public API reference

Generated rustdoc lives under `target/doc/` after `cargo doc --no-deps`.
This page is the human-curated highlight reel.

## `refrain_core`

```rust
pub fn parse(src: &str) -> Result<Refrain, RefrainError>
```

Parse a source string into a `Refrain` AST. Returns `RefrainError::Parse`
for malformed input.

```rust
pub struct Refrain {
    pub name: String,
    pub territorialize: Option<Pattern>,
    pub deterritorialize: Option<Pattern>,
    pub reterritorialize: Option<Pattern>,
}
```

```rust
pub enum Op {
    Note { pitch: String, dur: String },
    Loop { count: u32, body: Box<Pattern> },
    Diff { x: String, t: String },
    Quotient { rels: Vec<String> },
    Sym(String),
    Call { head: String, args: Vec<Pattern> },
}
```

## `refrain_egraph`

```rust
pub struct Egraph { /* ... */ }

impl Egraph {
    pub fn new() -> Self;
    pub fn with_limits(node_limit: usize, iter_limit: usize) -> Self;
    pub fn normalize(&self, r: &Refrain) -> Result<Refrain, RefrainError>;
}
```

Built-in rewrites: `(loop 1 ?x) → ?x`, `(seq ?x) → ?x`.

## `refrain_ffi` / Python `_native`

```python
from refrain_py import _native

j   = _native.parse_refrain("(refrain a (territorialize (note C4 q)))")
nor = _native.normalize_refrain(j)
ver = _native.version()  # "0.1.0"
```

## `intensity_plane`

```python
from intensity_plane import Dual, derivative, jacfwd, ehrhard_regnier_d, CellComplex

# Forward-mode autodiff.
d = derivative(lambda x: x * x, 3.0)   # → 6.0
j = jacfwd(lambda v: [v[0] * v[1], v[0] + v[1]], [2.0, 3.0])

# Symbolic differential on Refrain Pattern JSON.
d_pattern = ehrhard_regnier_d(pattern_json, "time")

# Cell complex.
c = CellComplex()
v0 = c.add(dim=0)
v1 = c.add(dim=0)
c.add(dim=1, boundary=(v0, v1))
print(c.euler_characteristic())  # 1
```

## `refrain_adapters`

```rust
pub trait RefrainAdapter: Send + Sync {
    fn name(&self) -> &str;
    fn emit(&self, refrain: &ExtractedRefrain, ctx: &EmitCtx)
        -> Result<Vec<u8>, AdapterErr>;
    fn capabilities(&self) -> AdapterCaps;
}
```

Built-in adapters: `AudioAdapter`, `VisualAdapter`, `CodeAdapter`,
`TextAdapter`. See the [Adapters](./adapters.md) chapter for details.

## `refrain_rhizome`

```rust
pub struct RhizomeBridge { /* ... */ }

impl RhizomeBridge {
    pub fn new() -> Self;
    pub fn alloc_eclass(&mut self) -> EClassId;
    pub fn insert(&mut self, causal: CausalId, eclass: EClassId);
    pub fn eclass_of(&self, causal: CausalId) -> Option<EClassId>;
    pub fn causal_events_of(&self, eclass: EClassId) -> &[CausalId];
    pub fn merge_classes(&mut self, a: EClassId, b: EClassId);
}
```

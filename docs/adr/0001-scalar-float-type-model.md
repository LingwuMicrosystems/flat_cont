# Add a floating-point scalar kind without redefining `Scalar`

`RawType::Scalar(u16)` was already understood as an integer scalar by both IR
layers and every consumer, so we added `RawType::Float(FloatKind)` and put the
kind on vector elements (`Vector { elem_bits, elem_kind }`) instead of replacing
`Scalar` with a kind-tagged variant. `FloatKind` mirrors the six target FPU
formats, while `FCond` only carries the four comparisons the target ISA encodes
directly; richer IEEE predicates wait for the target's NaN contract.

**Considered Options**: a unified `Scalar { bits, kind }` (cleaner but breaking
every existing `Scalar(n)` pattern and serialized value), and a `Float(u16)`
width-only kind (smaller but unable to distinguish `F16`/`Bf16` or `F32`/`P32`).

**Consequences**: `Scalar` stays a narrow, integer-only name; float vectors are
representable as types but remain rejected by the current code generator.

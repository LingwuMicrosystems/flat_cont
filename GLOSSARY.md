# flat_cont

The shared type and IR vocabulary for the flat-continuation program representation.

## Language

**RawType**:
The bit-layout type of a value or memory object.
_Avoid_: LLVM type, machine type

**Scalar**:
An integer scalar with a storage width in bits.
_Avoid_: Int, integer value

**Float**:
A floating-point scalar with a specific numeric format.
_Avoid_: FP scalar, floating value

**FloatKind**:
One of the six numeric formats `F16`, `F32`, `F64`, `Bf16`, `P32`, `P64`.
_Avoid_: FpuFormat, float width

**ElemKind**:
The element kind of a vector: integer or a specific `FloatKind`.
_Avoid_: lane type

**Vector**:
A fixed-length value whose elements share one `ElemKind` and storage width.
_Avoid_: SIMD value, packed scalar

**FCond**:
A floating-point comparison condition, limited to the four conditions the target ISA encodes directly.
_Avoid_: float condition code, IEEE predicate

**Fcmp**:
A floating-point comparison that produces a condition value.
_Avoid_: float compare, ordered comparison

**Convert**:
A change between two floating-point formats.
_Avoid_: cast, reinterpret

**Opcode**:
A generic operation mnemonic whose lowering depends on the operand types; float-only operations are separate opcodes.
_Avoid_: instruction, ISA opcode

use alloc::{boxed::Box, string::String, vec::Vec};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct DataId(pub u32);
#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ExternId(pub u32);

/// Floating-point format of a scalar or vector element.
///
/// The six variants mirror the target ISA's FPU formats. `flat_cont` defines
/// them locally so that the IR does not depend on the ISA crate.
#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FloatKind {
    F16,
    F32,
    F64,
    Bf16,
    P32,
    P64,
}

impl FloatKind {
    /// Storage width of this format in bits.
    pub const fn bits(self) -> u16 {
        match self {
            Self::F16 | Self::Bf16 => 16,
            Self::F32 | Self::P32 => 32,
            Self::F64 | Self::P64 => 64,
        }
    }
}

/// Element kind of a [`RawType::Vector`].
#[derive(Deserialize, Serialize, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElemKind {
    #[default]
    Int,
    Float(FloatKind),
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum RawType {
    Ptr,
    /// Integer scalar of the given bit width.
    Scalar(u16),
    /// Floating-point scalar.
    Float(FloatKind),
    Vector {
        elem_bits: u16,
        #[serde(default)]
        elem_kind: ElemKind,
    },
    Array(u32, Box<RawType>),
    Struct(Vec<RawType>),
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Visibility {
    Private,
    Public,
}

/// Function-level visibility / export name.
///
/// This is deliberately separate from [`Visibility`] (which `StaticData` uses):
/// `Public` carries the exported symbol name, so the variant enforces the
/// "public always names itself, private never does" invariant at the type level,
/// while [`Visibility`] stays a `Copy`/`repr(u8)` flag for data objects.
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum FunctionVisibility {
    /// Not exported.
    #[default]
    Private,
    /// Exported under the given (non-empty) symbol name.
    Public(String),
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct StaticData {
    pub name: String,
    pub visibility: Visibility,
    pub is_read_only: bool,
    pub alignment: u8,
    pub bytes: Vec<u8>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExternDecl {
    pub name: String,
    pub param: Vec<RawType>,
    pub ret: RawType,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Ordering {
    Relaxed,
    Release,
    Acquire,
    AcqRel,
    SeqCst,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum SyncScope {
    Group,
    Cluster,
    Device,
    #[default]
    System,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AtomicRMWCode {
    Swap,
    Add,
    Sub,
    And,
    Nand,
    Or,
    Xor,
    Max,
    Min,
    MaxSign,
    MinSign,
    Fadd,
    Fsub,
    Fmax,
    Fmin,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Opcode {
    Add,
    Sub,
    Mul,
    Div,
    DivSign,
    Rem,
    RemSign,
    ShiftLeft,
    ShiftRight,
    ShiftRightSign,
    And,
    Or,
    Xor,
    /// Unsigned integer minimum, or floating-point minimum.
    Min,
    /// Unsigned integer maximum, or floating-point maximum.
    Max,
    /// Signed integer minimum.
    MinSign,
    /// Signed integer maximum.
    MaxSign,
    /// Fused multiply-add: `a * b + c`.
    Fma,
    /// Fused multiply-subtract: `a * b - c`.
    Fms,
    /// Floating-point square root.
    Sqrt,
    /// Floating-point reciprocal square root.
    Rsqrt,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum ICond {
    Eq,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,
    GtSign,
    GeSign,
    LtSign,
    LeSign,
}

/// Floating-point comparison condition.
///
/// Deliberately smaller than [`ICond`]: it mirrors the target ISA's four
/// floating-point compare instructions. Ordered/unordered predicates and
/// `Ge`/`Le` are not modelled until the target's NaN contract is defined.
#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum FCond {
    Eq,
    Ne,
    Lt,
    Gt,
}

# AGENTS.md

`flat_cont`：一个 `no_std` Rust 库，定义 “flat continuation” IR（`src/flatcont.rs`
的 `FlatContGraph`）。它只定义数据结构和序列化，不负责从别的 IR 降级进来。

## 构建 / 测试
- `cargo build` / `cargo check` —— 编译 lib。
- `cargo test` —— 运行全部测试。
- 运行单个测试：`cargo test --test test_input_model`。

## 约定
- `#![no_std]`：集合类型从 `alloc::` 导入（`Vec`、`String`、`Box`），绝不用
  `std`。Edition 2024。
- 除非解释非显而易见的不变量，否则不加行内 `//` 注释；公开类型使用文档注释
  （`///`）。

## 架构
- `src/common.rs` —— IR 引用的类型词表（`RawType`、`Opcode`、`ICond`、`FCond`、
  `StaticData`、`DataId`、`ExternId` 等）。它描述的是名词，不是 IR 的形状。
- id 分居两处：`DataId`/`ExternId` 在 `common`；`ContId`/`FunId`/`ValueId`/
  `EffectId` 在 `flatcont`。
- 标量类型：`RawType::Scalar(u16)` 只表示整数标量；浮点标量用
  `RawType::Float(FloatKind)`（六种格式：F16/F32/F64/Bf16/P32/P64）。
  `Vector` 的元素种类由 `elem_kind` 区分（整数或浮点）。
- `FCond` 只对齐 ISA 直接编码的四个浮点比较（Eq/Ne/Lt/Gt）；`Ge`/`Le`、
  ordered/unordered 与 NaN 语义待浮点数值契约确定后再扩展。
- `src/flatcont.rs` —— 唯一的 IR 层：每个 `Function` 由自包含的
  `FlatContinuation` 组成，后者带值参数、effect 形参名与 effect 列表、
  dataflow 主体和一个 terminator。
- 操作数与 effect 是两套引用：`ValueRef` 指向值，`EffectPath` 指向 effect
  token。没有独立的投影节点。
- `EffectPath::Effect(EffectId)` 指向一个具体的 effect 节点，是路由与定序的
  依据；`EffectPath::NewEffect(Option<String>)` 标记「此处起一条新的 effect
  根」，不携带产生者 id，消费方按「无前驱依赖」处理。要指向某个 effect 节点时
  必须用前者。
- effect 的先后顺序由 `Effect(EffectId)` 与 `EffectMerge` 构成的引用链表达，
  不由 `NewEffect` 表达。`NewEffect` 携带的名字仅用于调试，不参与路由。
- 本 crate 没有把其他 IR 转换成 flat continuation 的 pass，也没有保留占位。
  删除的原因见 `docs/adr/0002-remove-basicblock-layer.md`。

## 说明
- `README.md` 和 `TODO.md`（中文设计文档）是在提交 `a6ba7ac` 里删除的；可用
  `git show a6ba7ac^:README.md` 找回。

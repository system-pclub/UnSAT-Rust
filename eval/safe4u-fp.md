# Safe4U False Positive 原因分析

本文逐个复核 `eval/safe4u-gpt4o` 与 `eval/safe4u-gpt5mini` 中模型实际判为
`No` 的安全约束。检查依据是 sample 源码、类型/字段可见性、safe API、状态
变化和调用顺序；没有使用已经过期的 `experiments.md`。

## 判定口径

- **TP**：模型判为 `No` 的具体安全约束确实能被安全代码可达的输入或状态
  破坏，模型指出的原因必须成立。
- **FP**：模型判为 `No` 的具体约束实际有保证。
- 同一 sample 中存在另一个真实 UB，不能为错误理由“兜底”。
- `pub`/`pub(crate)` safe API 和 crate 内可见类型上的 `pub` 字段均按安全可达
  处理，不能只检查当前生产调用点。

例如，下面的 `window` 是 TP，而不是 FP：

```rust
pub fn window(&self) -> &[u8] {
    unsafe { std::slice::from_raw_parts(self.start, self.window_len()) }
}
```

因为 `BufferWindow.start` 是 public，crate 内安全代码可把它设为 null；模型否定
的正是 `from_raw_parts` 的 non-null 约束。相反，如果模型否定的是一个实际上
恒成立的 alignment 或 one-past 约束，就不能用函数里的其他 bug 把它算成 TP。

## 汇总

| FP 类别 | 不同 sample 数 | GPT-4o FP | GPT-5-mini FP | FP 结果数 |
| --- | ---: | ---: | ---: | ---: |
| 1. 忽略类型、trait bound 和借用事实 | 3 | 2 | 1 | 3 |
| 2. 忽略真正不可绕过的 private 状态不变量 | 4 | 1 | 3 | 4 |
| 3. 缺少局部 guard / 值域关系推理 | 3 | 1 | 2 | 3 |
| 4. 忽略前序访问证明和 one-past 语义 | 5 | 1 | 4 | 5 |
| **合计** | **15** | **5** | **10** | **15** |

两个 run 合计 84 个 positive 结果，最终为：

- GPT-4o：31 TP / 5 FP；
- GPT-5-mini：38 TP / 10 FP；
- 合计：69 TP / 15 FP。

以下列出全部 15 个 FP，没有省略 sample。

## 1. 忽略类型、trait bound 和借用事实

### Safe4U 不太能处理的代码

unsafe 操作所需的事实有时编码在具体元素类型、泛型 bound 或 Rust 借用规则中，
而不是写成紧邻 unsafe 调用的动态检查。

代表代码（`stackvector-1.1.1/src/lib.rs`）：

```rust
impl<A: Array> StackVec<A>
where
    A::Item: Copy,
{
    pub fn from_slice(slice: &[A::Item]) -> Self {
        assert!(slice.len() <= A::size());
        // ...
        unsafe {
            ptr::copy_nonoverlapping(slice.as_ptr(), dst, slice.len());
        }
    }

    pub fn insert_from_slice(&mut self, index: usize, slice: &[A::Item]) {
        // `&mut self` 与 `slice` 不能由 safe Rust 构造成重叠借用。
        unsafe {
            ptr::copy_nonoverlapping(slice.as_ptr(), dst, slice.len());
        }
    }
}
```

### 全部 FP

| 模型 | sample_label | 模型否定的具体约束 | 为什么是 FP |
| --- | --- | --- | --- |
| GPT-4o | `cyberex-0.3.12/src/xffi/xtr.rs/string_to_buffer/2` | `copy_nonoverlapping` 的源、目标可能未对齐 | 拷贝元素类型是 `u8`，alignment 恒为 1。函数确实还存在目标不可写或重叠问题，但 GPT-4o 没有否定那些约束，不能为错误的 alignment 理由兜底。 |
| GPT-4o | `stackvector-1.1.1/src/lib.rs/from_slice/729` | 元素可能不是 `Copy`，复制后破坏类型语义 | 整个 impl 明确要求 `A::Item: Copy`；模型遗漏了直接写在定义上的 trait bound。 |
| GPT-5-mini | `stackvector-1.1.1/src/lib.rs/insert_from_slice/746` | 最终 `copy_nonoverlapping` 的源、目标可能重叠 | safe 调用必须同时持有 `&mut self` 和输入 `&[A::Item]`，借用规则不允许输入 slice 指向 `self` 的存储。public `length` 可导致其他 bounds/copy-length bug，但不能使这个不重叠约束失效。 |

## 2. 忽略真正不可绕过的 private 状态不变量

### Safe4U 不太能处理的代码

有些 unsafe 操作依赖真正 private 的字段。若 safe API 没有泄露该字段的可变引用，
模型不能把它当作任意外部输入。

代表代码（`jomini-0.27.0/src/binary/reader.rs`）：

```rust
pub struct TokenReader<R> {
    reader: R,
    buf: BufferWindow, // private
}

pub fn read_bytes(&mut self, bytes: usize) -> Result<&[u8], ReaderError> {
    while self.buf.window_len() < bytes {
        self.buf.fill_buf(&mut self.reader)?;
    }

    let input =
        unsafe { std::slice::from_raw_parts(self.buf.start, bytes) };
    self.buf.advance(bytes);
    Ok(input)
}
```

独立的 `BufferWindow` 因 public `start`/`end` 而不能信任；但
`TokenReader.buf` 本身是 private，正常到达目标 `from_raw_parts` 时，窗口关系由
reader 构造器和内部状态机维持。

### 全部 FP

| 模型 | sample_label | 模型否定的具体约束 | 为什么是 FP |
| --- | --- | --- | --- |
| GPT-5-mini | `jomini-0.27.0/src/binary/reader.rs/next/230` | `from_raw_parts` 的指针可能为 null 或未对齐 | `TokenReader.buf` 是 private；到达目标调用时 `start` 来自合法 slice/Box，且 `u8` alignment 为 1。恶意 `Read` 能在 refill 中触发另一个 pointer `add` 的 UB，但不是本约束。 |
| GPT-5-mini | `jomini-0.27.0/src/binary/reader.rs/read_bytes/93` | `from_raw_parts` 的长度可能大于 `isize::MAX` | 循环先保证 `window_len() >= bytes`；有效窗口 allocation 的长度本身不超过 `isize::MAX`。恶意 `Read` 的其他问题不能证明这个长度约束为 `No`。 |
| GPT-4o | `jomini-0.27.0/src/copyless.rs/init/15` | `as_mut_ptr().add(index)` 的 offset 可能不适合 `isize` | `VecAllocation` 字段 private，只能由 `VecHelper::alloc` 创建；它令 `index = vec.len()`，容量不足时先 `reserve(1)`，并持有 `&mut Vec<T>` 阻止 safe 代码破坏状态。 |
| GPT-5-mini | `jomini-0.27.0/src/text/reader.rs/read_bytes/510` | `from_raw_parts` 的长度可能大于 `isize::MAX` | 与 binary reader 相同，目标调用的 `bytes` 被有效 private 窗口长度限制。 |

## 3. 缺少局部 guard / 值域关系推理

### Safe4U 不太能处理的代码

unsafe 下标或 pointer offset 的范围由 helper 返回值、循环范围或多个 guard
共同推出。Safe4U 容易只寻找形式完全相同的 `index < len`。

代表代码（`i_triangle-0.26.0/src/delaunay/triangle.rs`）：

```rust
pub fn opposite(&self, neighbor: usize) -> usize {
    #[cfg(debug_assertions)]
    {
        for i in 0..3 {
            if self.neighbors[i] == neighbor {
                return i;
            }
        }
        panic!("Neighbor is not present");
    }

    #[cfg(not(debug_assertions))]
    {
        for i in 0..2 {
            if self.neighbors[i] == neighbor {
                return i;
            }
        }
        2
    }
}
```

### 全部 FP

| 模型 | sample_label | 模型否定的具体约束 | 为什么是 FP |
| --- | --- | --- | --- |
| GPT-5-mini | `i_triangle-0.26.0/src/delaunay/triangle.rs/update_opposite/117` | `neighbors.get_unchecked_mut(index)` 的下标可能不在 `0..3` | debug 路径只返回 `0..3` 或 panic；release 路径只返回 0、1、2。任意 neighbor 内容和参数都不能令目标下标越界。 |
| GPT-4o | `revm-interpreter-10.0.3/src/interpreter/stack.rs/exchange/268` | `as_mut_ptr().add(len - 1)` 的 offset 可能无效 | 到达该 `add` 的路径满足 `n + m < len` 且 `m > 0`，所以 `len > 0`，`len - 1` 是有效 Vec 下标。后面的 `sub` 确有 release overflow 风险，但 GPT-4o 没有否定那个约束。 |
| GPT-5-mini | `revm-interpreter-10.0.3/src/opcode/eof_printer.rs/print_eof_code/1` | `code.as_ptr().add(i + 2 + 2 * vtablei)` 可能越界或超过 `isize` | 循环给出 `vtablei < len`，前置 guard 给出 `i + 1 + 2 * len < code.len()`；合并后目标下标严格小于 `code.len()`。`len <= 256`，相关算术也不会先溢出。 |

## 4. 忽略前序访问证明和 one-past 语义

### Safe4U 不太能处理的代码

程序先读取当前 immediate/opcode，随后把 pointer 前移一或两个字节。若执行能无
UB 地到达后面的 `offset`，前序读取已经证明当前字节范围有效；Rust pointer
arithmetic 允许结果恰好等于 allocation 的 one-past pointer。

代表代码（`revm-interpreter-10.0.3/src/interpreter.rs`）：

```rust
let opcode = unsafe { *self.instruction_pointer };

// 当前 byte 已成功读取；+1 在 allocation 内或恰为 one-past。
self.instruction_pointer =
    unsafe { self.instruction_pointer.offset(1) };
```

把 `instruction_pointer` 设成无效值会在前序 dereference 就发生 UB，不能用来
证明模型点名的后续 `offset` 约束为 `No`。

### 全部 FP

| 模型 | sample_label | 模型否定的具体约束 | 为什么是 FP |
| --- | --- | --- | --- |
| GPT-4o | `revm-interpreter-10.0.3/src/instructions/data.rs/data_loadn/28` | `instruction_pointer.offset(2)` 可能超出 allocation | 前序 `read_u16` 已要求当前两个字节可读；到达目标时，前移 2 仍在 allocation 内或恰为 one-past。 |
| GPT-5-mini | `revm-interpreter-10.0.3/src/interpreter.rs/step/349` | `instruction_pointer.offset(1)` 可能超出 allocation | 目标前已成功解引用当前 opcode byte；前移 1 至多为 one-past。 |
| GPT-5-mini | `revm-interpreter-10.0.3/src/instructions/stack.rs/dupn/53` | pointer arithmetic 的 base/结果可能不在 allocation 范围 | 目标前已经读取当前 immediate byte，证明 base 有效；结果在 allocation 内或 one-past。 |
| GPT-5-mini | `revm-interpreter-10.0.3/src/instructions/stack.rs/exchange/73` | `instruction_pointer.offset(1)` 可能越过 allocation | 与 `dupn` 相同，当前 immediate 已在前序语句中成功读取。 |
| GPT-5-mini | `revm-interpreter-10.0.3/src/instructions/contract.rs/eofcreate/17` | 最后的 `instruction_pointer.offset(1)` 可能越过 allocation | 函数开头已读取当前 `initcontainer_index` byte，中间没有改变 pointer；无 UB 到达目标时前移 1 至多为 one-past。 |

## 从原 FP 改判为 TP 的条目

重新检查 visibility、任意 safe 调用和跨调用状态后，原来的 29 个 FP 结果中有
14 个应改判为 TP，涉及 8 个不同 sample：

- `i_triangle::Delaunay::build`：public triangle 字段可提供越界 index；
- `MSliceBuffer::new`：safe `MSlice::new` 可提供任意端点；
- `MSliceBuffer::add_connections`：先用长 slice、再用短 slice会遗留越界
  `edge.triangle`；
- `BufferWindow::{advance, window, get}`：public pointer 字段或参数可以直接破坏
  模型指出的 pointer 约束；
- `splay_tree::Tree::{node_ref, node_mut}`：crate 内可对空树直接调用 safe public
  helper。

逐 sample 的最终 TP/FP 判定与反例见 `eval/safe4u.md`。

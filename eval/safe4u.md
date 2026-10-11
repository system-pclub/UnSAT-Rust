# Safe4U TP/FP audit

本文件独立检查 `eval/safe4u-gpt4o` 和 `eval/safe4u-gpt5mini` 中的
positive 结果（`result = "unsound"`）。判定只使用：

- 每个 `results.json` 中模型实际判为 `No` 的安全约束及其回答；
- `samples_with_constraints.json` 中的 sample 源码；
- `benchmark/safe4u` 中的类型定义、可见性和调用链。

这里没有使用 `experiments.md` 的旧结论。

## 判定口径

- **TP**：模型判为 `No` 的具体安全约束确实能被安全代码可达的输入或状态
  破坏；模型指出的原因必须成立。
- **FP**：模型判为 `No` 的具体约束实际由类型、显式 guard，或不可由安全代码
  绕过的 private 状态不变量保证。同一 sample 内存在其他 UB，不能为错误理由
  “兜底”。
- `pub`/`pub(crate)` safe API 及 crate 内可见的 `pub` 字段均按安全可达处理，
  不能仅用“当前生产调用者不会这样调用”作为 FP 理由。safe trait（例如
  `Read`）的实现也不能被 unsafe 代码无条件信任其非类型化的行为约定。
- `—` 表示该模型对该 sample 返回 `sound`，不进入该模型的 TP/FP 统计；
  它不等价于 TN 或 FN。

## 汇总

| Run | 全部 sample | `unsound` | TP | FP | Positive precision |
| --- | ---: | ---: | ---: | ---: | ---: |
| GPT-4o | 226 | 62 | 47 | 15 | 75.81% |
| GPT-5-mini | 226 | 102 | 79 | 23 | 77.45% |
| 合计 | 452 | 164 | 126 | 38 | 76.83% |

`musli` 已从 benchmark 移除，其历史 eval 日志不进入统计。本轮保留的 9 个
新增 crate 共贡献 39 个 sample；同时此前未运行的 `arrow2` 产生了 89 个
sample，因此相对上一版每个模型共增加 128 个 sample。

## Google Sheets 数字

列顺序为 `crate,GPT-4o TP,GPT-4o FP,GPT-5-mini TP,GPT-5-mini FP`，行顺序
保持此前行顺序，并在末尾追加本轮 crate。

```csv
aegis,0,0,0,0
cyberex,2,1,3,0
fast-able,1,0,1,0
grubbnet,1,0,1,0
halo2curves-axiom,0,0,0,0
hugepage-rs,0,0,0,0
i_tree,2,0,2,0
i_triangle,3,0,6,1
jh,1,0,1,0
jomini,3,1,3,3
lurk-cli,1,0,1,0
meshopt,5,0,6,0
ptr_hash,2,0,2,0
qwt,0,0,0,0
stackvector,0,1,0,1
revm,6,2,7,5
roqoqo-quest,0,0,1,0
splay_tree,2,0,2,0
arrow2,8,8,14,6
bitm,2,0,2,0
memmap,1,0,2,0
fast-float,4,0,6,0
rust-crypto,0,0,7,2
parsepatch,1,0,1,0
falcon_capstone,0,0,1,1
hftbacktest,1,1,3,2
file_mmap,1,0,1,0
shaman,0,1,6,2
blade,0,0,0,0
```

两个模型合计有 110 个不同的 positive `sample_label`。以下逐一列出。

## 逐 sample 判定

### bitm-0.4.2

| sample_label | GPT-4o | GPT-5-mini | 独立检查 |
| --- | ---: | ---: | --- |
| `bitm-0.4.2/src/rank_select/mod.rs/try_rank/322` | TP | TP | 两个模型都质疑 `get_unchecked` 的下标。`RankSimple.content` 和 `ranks` 均为 public，安全调用者可构造长度关系不成立的值，使 `self.ranks.get_unchecked(block)` 越界。 |
| `bitm-0.4.2/src/rank_select/select.rs/select64/93` | TP | TP | `select64` 是 public safe API，未限制 `rank < n.count_ones()`；大 `rank` 可令 `SELECT_U8` 的 unchecked 下标超过表长。 |

### cyberex-0.3.12

| sample_label | GPT-4o | GPT-5-mini | 独立检查 |
| --- | ---: | ---: | --- |
| `cyberex-0.3.12/src/void.rs/delete/91` | TP | TP | public safe `delete<T>` 接受任意 `*mut c_void`，未证明其来自布局匹配的 `Box<T>`，可直接违反 `Box::from_raw` 合约。 |
| `cyberex-0.3.12/src/xffi/sto.rs/cchar_to_string/12` | TP | TP | 仅检查 null；安全调用者仍可传入 dangling、不可读或没有 NUL 终止符的指针，违反 `CStr::from_ptr` 合约。 |
| `cyberex-0.3.12/src/xffi/xtr.rs/string_to_buffer/2` | FP | TP | GPT-4o 唯一失败项是 alignment，但元素类型为 `u8`，alignment 恒为 1，所以该理由错误。GPT-5-mini 否定的目标可写性和不重叠约束则确实可由任意 public `buf` 指针破坏。 |

### fast-able-1.11.17

| sample_label | GPT-4o | GPT-5-mini | 独立检查 |
| --- | ---: | ---: | --- |
| `fast-able-1.11.17/src/vec.rs/get_uncheck/182` | TP | TP | public safe 方法把任意 `index` 直接交给 `get_unchecked`，没有 bounds guard。 |

### grubbnet-0.1.7

| sample_label | GPT-4o | GPT-5-mini | 独立检查 |
| --- | ---: | ---: | --- |
| `grubbnet-0.1.7/src/buffer.rs/drain/20` | TP | TP | `count` 没有检查，且 `NetworkBuffer.offset` 为 public；可令 `data.as_ptr().offset(count)` 越过固定数组，或令 copy 长度失效。 |

### i_tree-0.9.0

| sample_label | GPT-4o | GPT-5-mini | 独立检查 |
| --- | ---: | ---: | --- |
| `i_tree-0.9.0/src/tree.rs/mut_node/69` | TP | TP | public safe 方法接受任意 `u32`，直接用于 `store.buffer.get_unchecked_mut`。 |
| `i_tree-0.9.0/src/tree.rs/node/62` | TP | TP | public safe 方法接受任意 `u32`，直接用于 `store.buffer.get_unchecked`。 |

### i_triangle-0.26.0

| sample_label | GPT-4o | GPT-5-mini | 独立检查 |
| --- | ---: | ---: | --- |
| `i_triangle-0.26.0/src/delaunay/delaunay.rs/build/56` | — | TP | `Delaunay.triangles`、`DTriangle.index` 和 `DTriangle.neighbors` 均为 public，`build` 为 safe `pub(crate)`。crate 内安全代码可写入越界 neighbor/index 后调用 `build`，使 `get_unchecked(_mut)` 越界；不能只依赖当前生产调用者构造的三角形。 |
| `i_triangle-0.26.0/src/delaunay/delaunay.rs/to_triangulation/13` | — | TP | `Delaunay.triangles`、`DTriangle.vertices` 和 `DVertex.index` 可由安全代码设置。令 vertex index 为 `usize::MAX`，release 下 `max_index + 1` 回绕，随后 `p_pnt.add(a.index)` 的字节偏移不适合 `isize`。 |
| `i_triangle-0.26.0/src/delaunay/triangle.rs/neighbor_by_order/10` | TP | TP | public safe `order` 未限制为 `0..3`。 |
| `i_triangle-0.26.0/src/delaunay/triangle.rs/update_opposite/117` | — | FP | `opposite` 在 debug 中找不到 neighbor 会 panic，在 release 中只返回 `0`、`1` 或 `2`；交给三元素数组的 unchecked 下标始终有效。 |
| `i_triangle-0.26.0/src/delaunay/triangle.rs/vertex_by_order/16` | TP | TP | public safe `order` 未限制为 `0..3`。 |
| `i_triangle-0.26.0/src/monotone/mslice_buffer.rs/add_connections/60` | — | TP | safe public 方法可在同一 `MSliceBuffer` 上重复调用。第一次用较长 triangle slice 可把 `edge.triangle` 记为较大有效 `i`；第二次传入较短 slice 时，该旧值未经重新校验就进入 `triangles.get_unchecked_mut(edge.triangle)`，可安全触发越界。 |
| `i_triangle-0.26.0/src/monotone/mslice_buffer.rs/new/37` | TP | TP | `MSlice::new(a, b)` 和 `MSliceBuffer::new(vertex_count, slices)` 都可由 crate 内安全代码调用，且前者不检查端点。令 `a >= vertex_count` 即可使 `vertex_marks.get_unchecked_mut(a)` 越界；唯一生产调用的正常参数不能覆盖这个安全可达反例。 |

### jh-0.1.0

| sample_label | GPT-4o | GPT-5-mini | 独立检查 |
| --- | ---: | ---: | --- |
| `jh-0.1.0/src/compressor.rs/f8_impl/97` | TP | TP | `f8_impl` 是 public safe 函数并接受任意 raw `data` 指针，无法保证四个 SIMD word 的可读性、初始化和 pointer-offset 合约。 |

### jomini-0.27.0

| sample_label | GPT-4o | GPT-5-mini | 独立检查 |
| --- | ---: | ---: | --- |
| `jomini-0.27.0/src/binary/reader.rs/next/230` | — | FP | 模型否定的是目标 `from_raw_parts` 的 non-null/alignment；`TokenReader.buf` 为 private，正常到达该调用时 `start` 来自 slice/Box，且 `u8` alignment 为 1。恶意 `Read` 可在 refill 的另一个 pointer `add` 处制造 UB，但不能为这条错误理由兜底。 |
| `jomini-0.27.0/src/binary/reader.rs/read_bytes/93` | — | FP | 模型否定的是 `from_raw_parts` 的 `len <= isize::MAX`。到达目标调用时循环已保证 `window_len() >= bytes`，有效窗口 allocation 的长度不超过 `isize::MAX`。恶意 `Read` 的问题发生在另一个 unsafe `add`，不是该约束。 |
| `jomini-0.27.0/src/buffer.rs/advance/43` | TP | TP | `BufferWindow.start`/`end` 为 public，`advance` 是 safe public 方法（类型在 crate 内可见）。安全代码可把 `start` 改成任意 raw pointer，或传入超过当前 allocation 的 `amt`，使 `start.add(amt)` 违反 in-bounds/no-wrap 约束。release 下后置 `debug_assert!` 也不是前置保护。 |
| `jomini-0.27.0/src/buffer.rs/get/70` | TP | TP | `get` 是 safe public 方法，并直接接受任意 `Range<*const u8>`。crate 内安全调用者可传入不同 allocation、逆序、null 或 dangling 的两端；`debug_assert!` 在 release 中不提供保证，因此 `offset_from`/`from_raw_parts` 的失败约束确实可达。 |
| `jomini-0.27.0/src/buffer.rs/window/50` | TP | TP | `BufferWindow.start` 和 `end` 都是 public，crate 内安全代码可将它们改成任意值后调用 safe `window()`。因此 `from_raw_parts` 的 non-null、同 allocation 和长度约束均无法由构造器保证。 |
| `jomini-0.27.0/src/copyless.rs/init/15` | FP | — | `VecAllocation` 字段为 private，只能由 `VecHelper::alloc` 创建；它先 reserve，并令 `index = len`。有效 Vec allocation 已保证 `index * size_of::<T>() <= isize::MAX`。 |
| `jomini-0.27.0/src/text/reader.rs/read_bytes/510` | — | FP | 与 binary reader 相同，模型否定的是 slice 长度上界；目标调用的 `bytes` 已被有效窗口长度限制。恶意 `Read` 能触发的是 refill 中另一个 `add` 的 UB，不能证明本约束为 `No`。 |

### lurk-cli-0.3.8

| sample_label | GPT-4o | GPT-5-mini | 独立检查 |
| --- | ---: | ---: | --- |
| `lurk-cli-0.3.8/src/arch/mod.rs/read_string/69` | TP | TP | 指针由被跟踪进程的整数地址转换而来，不是当前进程 allocation 的派生指针；第二次及之后的非零 `offset(count)` 可违反 pointer provenance/range 合约。 |

### meshopt-0.3.0

| sample_label | GPT-4o | GPT-5-mini | 独立检查 |
| --- | ---: | ---: | --- |
| `meshopt-0.3.0/src/optimize.rs/optimize_overdraw_in_place/158` | TP | TP | `VertexDataAdapter.position_offset` 为 public，可安全构造为超过 `reader` slice 的值，使 `vertex_data.add(position_offset)` 违规。 |
| `meshopt-0.3.0/src/shadow.rs/generate_shadow_indices/8` | TP | TP | 同上，public `position_offset` 可破坏构造器原本建立的范围不变量。 |
| `meshopt-0.3.0/src/simplify.rs/simplify/28` | — | TP | 同上。 |
| `meshopt-0.3.0/src/simplify.rs/simplify_sloppy/188` | TP | TP | 同上。 |
| `meshopt-0.3.0/src/simplify.rs/simplify_with_locks/103` | TP | TP | 同上。 |
| `meshopt-0.3.0/src/utilities.rs/pos_ptr/184` | TP | TP | 方法本身直接使用 public `position_offset`；安全调用者无需经过 `new` 即可构造不满足 offset 范围的 adapter。 |

### ptr_hash-0.1.1

| sample_label | GPT-4o | GPT-5-mini | 独立检查 |
| --- | ---: | ---: | --- |
| `ptr_hash-0.1.1/src/tiny_ef.rs/index/33` | TP | TP | public safe `index` 没有检查 `index / 44 < ef.len()`。 |
| `ptr_hash-0.1.1/src/util.rs/prefetch_index/9` | TP | TP | public safe 函数允许任意 `index`；`s.as_ptr().add(index)` 在 `index > s.len()` 时已产生 UB，与 prefetch 是否真正解引用无关。 |

### revm-interpreter-10.0.3

| sample_label | GPT-4o | GPT-5-mini | 独立检查 |
| --- | ---: | ---: | --- |
| `revm-interpreter-10.0.3/src/instructions/contract.rs/eofcreate/17` | — | FP | 模型否定的是最后的 `offset(1)`。在该调用前已经成功读取当前 immediate byte；若能无 UB 地到达目标，base 指向有效 `u8`，加一至多为 one-past。任意无效 pointer 会先在 dereference 处 UB，不能证明这个 offset 理由正确。 |
| `revm-interpreter-10.0.3/src/instructions/control.rs/rjump/7` | TP | TP | instruction 函数和 `Interpreter` 指针字段均为 public。安全调用者可提供两字节有效 immediate，但让其中的 signed jump offset 跳出 bytecode allocation。 |
| `revm-interpreter-10.0.3/src/instructions/control.rs/rjumpi/16` | TP | TP | 条件跳转的 offset 可由 immediate 控制，public `Interpreter` 状态也未强制经过 EOF verifier；可在成功读取 immediate 后跳出 allocation。 |
| `revm-interpreter-10.0.3/src/instructions/control.rs/rjumpv/30` | TP | TP | public instruction API 可在未经过 EOF 验证的状态上调用；表长/表项可令中间或最终 pointer offset 越界。 |
| `revm-interpreter-10.0.3/src/instructions/data.rs/data_loadn/28` | FP | — | `read_u16` 在目标前已要求当前两字节可读；随后 `offset(2)` 正好位于这两个字节之后，至多为 one-past。无效 pointer 会先在读取中 UB，不能证明模型否定的 offset range 约束。 |
| `revm-interpreter-10.0.3/src/instructions/stack.rs/dupn/53` | — | FP | 目标 `offset(1)` 前已经解引用当前 immediate byte；无 UB 到达目标时，base 有效且加一至多为 one-past。 |
| `revm-interpreter-10.0.3/src/instructions/stack.rs/exchange/73` | — | FP | 与 `dupn` 相同：模型否定的是后续 `offset(1)`，但前序成功读取已证明该调用所需的一字节范围。 |
| `revm-interpreter-10.0.3/src/instructions/stack.rs/push/24` | TP | TP | instruction 函数和 `Interpreter.instruction_pointer` 都是 public；安全调用者可设置无效/过短指针或选择过大的 const `N`，违反 `from_raw_parts(ip, N)` 及后续 `add(N)` 合约。 |
| `revm-interpreter-10.0.3/src/interpreter.rs/load_eof_code/125` | TP | TP | 虽为 crate-private，但 public `retf` 可达；`Interpreter.function_stack.return_stack` 和 frame 的 `pc` 均为 public，可放入有效 code index 与任意大 `pc`，使 `bytecode.as_ptr().add(pc)` 违规。 |
| `revm-interpreter-10.0.3/src/interpreter.rs/program_counter/339` | — | TP | `instruction_pointer` 为 public；安全调用者可令它与 `bytecode.as_ptr()` 不属于同一 allocation，甚至让绝对地址距离超过 `isize::MAX`，直接违反 `offset_from` 合约。 |
| `revm-interpreter-10.0.3/src/interpreter.rs/step/349` | — | FP | 模型否定的是读取 opcode 后的 `offset(1)`；若能无 UB 到达该调用，当前 byte 已被成功解引用，所以结果在 allocation 内或 one-past。公开 pointer 导致的前序 dereference UB 是另一个约束。 |
| `revm-interpreter-10.0.3/src/interpreter/stack.rs/exchange/268` | FP | TP | GPT-4o 只否定 `as_mut_ptr().add(len - 1)`；成功路径的 guard 已推出 `len > 0`，该 add 有效。GPT-5-mini 否定后面的 `sub`，release 下 `n + m` 可回绕后通过 guard，原始大 `n` 的确使 `top.sub(n)` 违规。 |
| `revm-interpreter-10.0.3/src/interpreter/stack.rs/push_slice/290` | TP | — | `Stack::data_mut` 公开返回 `&mut Vec`，安全调用者可 shrink/替换其容量，破坏 `capacity == STACK_LIMIT` 不变量；随后 `set_len(new_len)` 可超过实际 capacity。 |
| `revm-interpreter-10.0.3/src/opcode/eof_printer.rs/print_eof_code/1` | — | FP | `vtablei < len`，且前置检查要求 `i + 1 + 2 * len < code.len()`；所以 `i + 2 + 2 * vtablei` 严格落在有效 slice 内，也必然不超过 `isize::MAX`。 |

### roqoqo-quest-0.14.4

| sample_label | GPT-4o | GPT-5-mini | 独立检查 |
| --- | ---: | ---: | --- |
| `roqoqo-quest-0.14.4/src/quest_bindings.rs/set/258` | — | TP | `ComplexMatrixN.complex_matrix` 和 `dimension` 均为 public；安全代码可伪造过大的 dimension/row 或底层 pointer table，使 `pointer.add(row/column)` 的偏移溢出或越界。 |

### splay_tree-0.3.1

| sample_label | GPT-4o | GPT-5-mini | 独立检查 |
| --- | ---: | ---: | --- |
| `splay_tree-0.3.1/src/tree_core.rs/node_mut/423` | TP | TP | 虽然 `tree_core` 不对下游 crate 导出，但 `Tree::new` 和 `node_mut` 都是 safe public 方法，crate 内安全代码可创建空树并直接调用 `node_mut(0)`。因此 unchecked mutable index 的越界反例安全可达。 |
| `splay_tree-0.3.1/src/tree_core.rs/node_ref/420` | TP | TP | 与 `node_mut` 相同：crate 内安全代码可对空的 `Tree` 直接调用 safe `node_ref(0)`；不能只检查当前公开 wrapper 是否先做 bounds check。 |

### stackvector-1.1.1

| sample_label | GPT-4o | GPT-5-mini | 独立检查 |
| --- | ---: | ---: | --- |
| `stackvector-1.1.1/src/lib.rs/from_slice/729` | FP | — | GPT-4o 失败项声称非 `Copy` 元素会违规，但整个 impl 明确要求 `A::Item: Copy`；长度 assert 和 unsafe `Array` 合约也保证目标空间。 |
| `stackvector-1.1.1/src/lib.rs/insert_from_slice/746` | — | FP | GPT-5-mini 否定的是最终 `copy_nonoverlapping` 的源/目标不重叠。安全调用必须同时持有 `&mut self` 和输入 slice，借用规则不允许 slice 指向 `self` 的存储；public `length` 可引出其他 bounds/copy-length bug，但不能使该不重叠理由成立。 |

### arrow2-0.18.0

| sample_label | GPT-4o | GPT-5-mini | 独立检查 |
| --- | ---: | ---: | --- |
| `arrow2-0.18.0/src/offset.rs/last/145` | TP | TP | safe `Offsets::new_zeroed(usize::MAX)` 中的 `length + 1` 可在 release 回绕为 0，从而构造空 offsets；随后 `last()` 能到达 `unreachable_unchecked`。 |
| `arrow2-0.18.0/src/offset.rs/start_end/156` | — | TP | 同一 safe 路径可构造空 offsets；此时 `len_proxy()` 的 `0 - 1` 在 release 回绕为 `usize::MAX`，让 `index = 0` 通过 assert 后进入空 Vec 的 unchecked access。 |
| `arrow2-0.18.0/src/array/null.rs/slice/60` | — | TP | 抽取出的约束要求严格 `< self.len()`，而 safe wrapper 明确允许 `offset + length == self.len()`；例如对整个数组切片即可违反该具体约束。 |
| `arrow2-0.18.0/src/array/boolean/mod.rs/get/151` | FP | FP | `Array::is_null(i)` 内部先执行 `assert!(i < self.len())`，越界时会在到达 `value_unchecked` 前 panic。 |
| `arrow2-0.18.0/src/array/binary/mod.rs/get/160` | FP | FP | 与 boolean array 相同，`is_null(i)` 已完成模型声称缺失的 bounds check。 |
| `arrow2-0.18.0/src/array/utf8/mod.rs/get/179` | FP | FP | `is_null(i)` 保证 `i < self.len()`；模型忽略了 trait 默认方法中的显式 assert。 |
| `arrow2-0.18.0/src/array/utf8/mod.rs/into_mut/256` | FP | — | `Utf8Array` 的 safe 构造器验证 offsets，字段为 private；`into_mut` 只转移或取得同一 buffer 的所有权，不会破坏单调性。 |
| `arrow2-0.18.0/src/array/fixed_size_binary/mod.rs/get/165` | FP | FP | 越界 `i` 会先在 `is_null(i)` 中 panic，无法到达 `value_unchecked(i)`。 |
| `arrow2-0.18.0/src/array/growable/utils.rs/build_extend_null_bits/6` | TP | TP | public safe `Growable::extend` 接受任意 `index/start/len`；当源 array 有 validity bitmap 时，closure 在后续 safe slice panic 之前先用这些未验证值调用 `extend_from_slice_unchecked`。 |
| `arrow2-0.18.0/src/array/primitive/mod.rs/get/204` | FP | FP | `is_null(i)` 的 assert 已建立 `value_unchecked` 所需的下标界限。 |
| `arrow2-0.18.0/src/array/map/mod.rs/slice/114` | — | TP | 抽取约束使用严格 `<`，wrapper 只检查 `<=`，所以末端切片会通过 assert 却违反该约束；此外 `offset + length` 在 release 下还可回绕。 |
| `arrow2-0.18.0/src/array/list/mod.rs/slice/113` | — | TP | 与 `MapArray::slice` 相同，safe wrapper 允许等于逻辑长度，而抽取约束要求严格小于。 |
| `arrow2-0.18.0/src/array/fixed_size_list/mod.rs/get/175` | FP | FP | `is_null(i)` 会先检查 `i < self.len()`，模型的缺少 bounds check 结论不成立。 |
| `arrow2-0.18.0/src/array/dictionary/mod.rs/key_value/368` | TP | TP | safe `try_new` 在 `keys.null_count() == keys.len()` 时跳过全部 key 验证；安全代码可构造底层值为 `-1`、validity 全 false 的 signed-key array，再由 `key_value(0)` 调用失败的 `as_usize`。 |
| `arrow2-0.18.0/src/array/dictionary/mod.rs/value/386` | TP | TP | 与 `key_value` 相同，全 null 会让负的底层 key 绕过构造器验证；`value(0)` 仍读取该 key 并调用 `as_usize`。 |
| `arrow2-0.18.0/src/array/dictionary/value_map.rs/from_values/80` | FP | — | map 中每个 key 都由当前循环中已验证的 `index < values.len()` 转换而来；构造过程中 values 不会改变，已有 key 始终在界内。 |
| `arrow2-0.18.0/src/array/dictionary/value_map.rs/try_push_valid/133` | TP | TP | vacant 分支先向 map 插 key，再调用可失败的 safe `push` closure；若 closure 返回 `Err` 且未追加值，map 会残留等于旧长度的 key，下一次 safe 调用可把它交给 `value_unchecked_at`。 |
| `arrow2-0.18.0/src/bitmap/immutable.rs/sliced/199` | — | TP | 抽取约束包含已有的 `self.offset`，而 wrapper 只检查相对当前 view 的 `offset + length <= self.length`；对已有非零 offset 的合法 bitmap 做整段切片即可违反该具体约束。 |
| `arrow2-0.18.0/src/buffer/immutable.rs/as_slice/107` | TP | — | safe `Buffer::slice/sliced` 的 `offset + length` guard 可在 release 下回绕后通过，继而写入不合法的 private offset/length；随后 `as_slice` 的 unchecked range 会越界。 |
| `arrow2-0.18.0/src/buffer/immutable.rs/get_mut_slice/230` | — | TP | 同一个 safe slice 回绕路径可破坏 buffer 范围不变量；若底层 `Arc<Vec<_>>` 唯一，`get_mut_slice` 会把该非法 range 直接交给 `get_unchecked_mut`。 |
| `arrow2-0.18.0/src/ffi/schema.rs/format/147` | TP | TP | `ArrowSchema.format` 是 `pub(super)` raw field；按本文 crate 内可见字段口径，`ffi` 模块中的安全代码可构造非 null、但 dangling 或无 NUL 的值并调用 safe `format()`。 |
| `arrow2-0.18.0/src/ffi/schema.rs/name/158` | TP | TP | `name` 同样是 crate 内安全可写的 `pub(super)` raw field；null guard 不能保证非 null 值可读且有 NUL 结尾。 |
| `arrow2-0.18.0/src/ffi/schema.rs/child/166` | — | TP | `children` 与 `n_children` 都是 `pub(super)`；crate 内安全代码可令 count 大于零而 pointer 无效，使 safe `child(0)` 的 `add` 违反 allocation/range 合约。 |

### falcon_capstone-0.5.2

| sample_label | GPT-4o | GPT-5-mini | 独立检查 |
| --- | ---: | ---: | --- |
| `falcon_capstone-0.5.2/src/capstone.rs/get/385` | — | TP | safe public `InstrBuf::new` 接受任意 raw pointer 和 `count`；调用者可令 `index < count`，同时让 `ptr.add(index)` 溢出地址空间或离开 allocation。 |
| `falcon_capstone-0.5.2/src/capstone.rs/reg_name/559` | — | FP | 对 safe 构造出的 Capstone handle，`cs_reg_name` 的 C API 返回 null 或有效的库管理字符串；不能只因返回类型是 raw pointer 就否定其 FFI 合约。 |

### fast-float-0.2.0

| sample_label | GPT-4o | GPT-5-mini | 独立检查 |
| --- | ---: | ---: | --- |
| `fast-float-0.2.0/src/common.rs/step_by/21` | TP | TP | safe `step_by` 接受任意 `usize`，没有用 private `end` 限制 `n`，可直接使 `ptr.add(n)` 离开原 slice allocation。 |
| `fast-float-0.2.0/src/common.rs/check_len/76` | TP | TP | 这个“检查”先计算 `self.ptr.add(n)`；任意大 `n` 会在比较发生前违反 pointer-add 合约。 |
| `fast-float-0.2.0/src/common.rs/read_u64/90` | TP | TP | safe public 方法只有 `debug_assert!`，release 下可对短于 8 字节的 `AsciiStr` 调用并让 copy 从源 slice 越界读取。 |
| `fast-float-0.2.0/src/common.rs/get_at/108` | TP | TP | safe `ByteSlice::get_at` 没有 bounds guard，crate 内安全代码可在任意短 slice 上传入越界 `i`。 |
| `fast-float-0.2.0/src/common.rs/read_u64/159` | — | TP | safe trait 默认方法同样只用 `debug_assert!`；release 下短 slice 会使 8-byte copy 越界。 |
| `fast-float-0.2.0/src/common.rs/write_u64/169` | — | TP | safe trait 默认方法可对短 mutable slice 调用，8-byte copy 会超过目标范围。 |

### file_mmap-0.20.0

| sample_label | GPT-4o | GPT-5-mini | 独立检查 |
| --- | ---: | ---: | --- |
| `file_mmap-0.20.0/src/lib.rs/write/82` | TP | TP | safe public `write` 接受任意 signed `addr` 和长度，未对 mmap 范围做检查；负值、过大偏移或过长 slice 都能破坏 `offset`/`from_raw_parts_mut` 合约。 |

### hftbacktest-0.4.0

| sample_label | GPT-4o | GPT-5-mini | 独立检查 |
| --- | ---: | ---: | --- |
| `hftbacktest-0.4.0/src/backtest/mod.rs/goto/1136` | — | FP | `ev.asset_no` 由 private `EventSet` 中长度为 `4 * num_assets` 的 timestamp array 下标右移两位得到；`Backtest` 构造器同时保证 local/exchange 长度为 `num_assets`。 |
| `hftbacktest-0.4.0/src/backtest/evs.rs/next/48` | — | FP | `EventSet::new` 拒绝零 assets，并创建非空 timestamp array；字段为 private，安全代码无法令 `get_unchecked(0)` 面对空数组。 |
| `hftbacktest-0.4.0/src/backtest/data/mod.rs/at/159` | TP | TP | safe public `DataPtr::at` 接受任意 index，没有用 fat pointer 的 `len` 做 guard，可令 `ptr.add(index)` 越界。 |
| `hftbacktest-0.4.0/src/backtest/data/npy/mod.rs/read_npy/167` | — | TP | safe generic API 只要求文档为 C POD 的 unsafe `POD` trait，并未约束 `align_of::<D>() <= CACHE_LINE_SIZE`；合法的高对齐 POD 可使固定 64-byte 对齐 buffer 不满足 `Data<D>` 的布局要求。 |
| `hftbacktest-0.4.0/src/utils/aligned.rs/new/37` | FP | TP | GPT-4o 只否定“必须释放”，但 `Drop` 会按同一 layout dealloc。GPT-5-mini 否定 layout 则成立：public const `ALIGNMENT` 可取非二次幂，且 `len * size_of::<T>()` 的 guard 本身可回绕。 |

### memmap-0.7.0

| sample_label | GPT-4o | GPT-5-mini | 独立检查 |
| --- | ---: | ---: | --- |
| `memmap-0.7.0/src/unix.rs/flush/120` | — | TP | public safe `flush_range` 原样转发任意 offset/len；页面对齐计算不证明结果仍位于 mmap allocation，超大输入还会发生截断或回绕。 |
| `memmap-0.7.0/src/unix.rs/flush_async/133` | TP | TP | 与同步版本相同，safe range API 没有检查 `offset + len <= map.len()`；任意大 offset 可让 cast 后的 pointer offset 越界或环绕。 |

### parsepatch-0.3.1

| sample_label | GPT-4o | GPT-5-mini | 独立检查 |
| --- | ---: | ---: | --- |
| `parsepatch-0.3.1/src/parser.rs/parse_numbers/179` | TP | TP | safe `parse_numbers` 可在不足 4 字节的当前 line 上调用，注释中的“已知前缀”没有类型或 guard 保证，`get_unchecked(4..)` 可越界。 |

### rust-crypto-0.2.36

| sample_label | GPT-4o | GPT-5-mini | 独立检查 |
| --- | ---: | ---: | --- |
| `rust-crypto-0.2.36/src/cryptoutil.rs/write_u64v_le/41` | — | TP | 长度等式允许两个 slice 都为空，但函数仍在循环前执行 `get_unchecked(_mut)(0)`，空输入即可安全触发越界。 |
| `rust-crypto-0.2.36/src/cryptoutil.rs/write_u32v_le/78` | — | TP | 同上，空的 dst/input 满足 assert，却对两者取 unchecked 第零项。 |
| `rust-crypto-0.2.36/src/cryptoutil.rs/read_u64v_be/93` | — | TP | 空 dst/input 满足 `dst.len() * 8 == input.len()`，随后两个 unchecked index 仍越界。 |
| `rust-crypto-0.2.36/src/cryptoutil.rs/read_u64v_le/109` | — | TP | 与 big-endian 版本相同，零长度反例成立。 |
| `rust-crypto-0.2.36/src/cryptoutil.rs/read_u32v_be/125` | — | TP | 长度 guard 没有排除空 slice，循环前的 unchecked index 违反合约。 |
| `rust-crypto-0.2.36/src/cryptoutil.rs/read_u32v_le/141` | — | TP | 与 big-endian 版本相同，空 slice 即为安全可达反例。 |
| `rust-crypto-0.2.36/src/cryptoutil.rs/xor_keystream/177` | — | FP | safe Rust slice allocation 的字节长度不超过 `isize::MAX`，所以 `plaintext.len() as isize` 不会截断；循环中的 offset 也始终小于 slice 长度。 |
| `rust-crypto-0.2.36/src/cryptoutil.rs/copy_memory/192` | — | FP | 同一 safe 调用中 `src: &[u8]` 与 `dst: &mut [u8]` 不能重叠；长度 assert 又保证目标足够大。 |
| `rust-crypto-0.2.36/src/util.rs/fixed_time_eq/42` | — | TP | 两个空 slice 长度相等并进入 unsafe 分支，随后 `get_unchecked(0)` 对二者都越界，即使外部汇编的 count 为零。 |

### shaman-0.1.0

| sample_label | GPT-4o | GPT-5-mini | 独立检查 |
| --- | ---: | ---: | --- |
| `shaman-0.1.0/src/cryptoutil.rs/write_u64_be/20` | FP | — | GPT-4o 唯一否定项是 destination alignment，但 `dst.get_unchecked_mut(0)` 的元素类型为 `u8`，alignment 恒为 1。 |
| `shaman-0.1.0/src/cryptoutil.rs/write_u64v_le/41` | — | TP | 空 dst/input 满足长度 assert，但循环前的两个 unchecked 第零项仍越界。 |
| `shaman-0.1.0/src/cryptoutil.rs/write_u32v_le/78` | — | TP | 与 u64 版本相同，零长度输入是直接反例。 |
| `shaman-0.1.0/src/cryptoutil.rs/read_u64v_be/93` | — | TP | 空 slice 满足长度等式，随后 `get_unchecked(_mut)(0)` 越界。 |
| `shaman-0.1.0/src/cryptoutil.rs/read_u64v_le/109` | — | TP | 与 big-endian 版本相同，零长度反例成立。 |
| `shaman-0.1.0/src/cryptoutil.rs/read_u32v_be/125` | — | TP | 长度 guard 允许两个 slice 为空，不能保护 unchecked 第零项。 |
| `shaman-0.1.0/src/cryptoutil.rs/read_u32v_le/141` | — | TP | 与 big-endian 版本相同。 |
| `shaman-0.1.0/src/cryptoutil.rs/xor_keystream/177` | — | FP | Rust slice 长度受 `isize::MAX` 限制，cast 不会产生模型声称的 offset overflow，循环下标始终在三个 slice 的已检查范围内。 |
| `shaman-0.1.0/src/cryptoutil.rs/copy_memory/192` | — | FP | safe 借用规则保证 `src` 与 `dst` 不重叠，目标长度也由 assert 保证。 |

## 没有 positive 的 crate

以下五个 crate 在两个 run 中均没有 `unsound` sample，因此上表没有对应行：

- `aegis-0.9.12`
- `blade-0.2.0`（两个 run 都是 0 sample）
- `halo2curves-axiom-0.7.0`
- `hugepage-rs-0.1.0`
- `qwt-0.2.0`

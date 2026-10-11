# RukLee TP audit

本文记录 `eval/ruklee-full` 中经独立检查确认的 RukLee true positives，仅保留
TP 统计和具体依据。

## 统计口径

- 颗粒度为函数：同一函数的多个 callsite/rule 合并为一行。
- TP 要求 RukLee 报出的具体 rule 至少有一条可被安全代码可达的输入或状态破坏。
- `pub`/`pub(crate)` safe API、crate 内可见类型的 public 字段、重复调用产生的
  状态，以及 safe trait 实现均按安全可达处理。
- `low-confidence-sat`、`timeout`、`verified`、`not-in-ir` 等非 violation
  状态不计入 TP。
- `musli` 已从 benchmark 移除。

## 汇总

| Run | TP functions |
| --- | ---: |
| RukLee full | 95 |

## Google Sheets 数字

列顺序为 `crate,RukLee TP`，行顺序与 Safe4U 表相同。数值 0 表示当前没有确认的
TP，不表示已经证明该 crate 安全。

```csv
crate,RukLee TP
aegis,8
cyberex,5
fast-able,1
grubbnet,1
halo2curves-axiom,1
hugepage-rs,3
i_tree,2
i_triangle,5
jh,1
jomini,6
lurk-cli,1
meshopt,6
ptr_hash,2
qwt,1
stackvector,10
revm,11
roqoqo-quest,1
splay_tree,2
arrow2,1
bitm,2
memmap,1
fast-float,1
rust-crypto,10
parsepatch,1
falcon_capstone,1
hftbacktest,1
file_mmap,1
shaman,8
blade,1
```

## 逐函数判定

### aegis-0.9.12

| Function | Result | 独立检查 |
| --- | ---: | --- |
| `c::aegis128l::Aegis128LMac::new` | TP | `MaybeUninit::assume_init` 要求 state 已被完整初始化；此前只有一次 `extern` 的 `aegis128l_mac_init(st.as_mut_ptr(), ...)` 调用。Rust 无法从该声明证明外部实现一定写满 state；若实现不写或只写部分字段，safe constructor 随后即对未初始化值执行 `assume_init`。 |
| `c::aegis128l::Aegis128LMac::new_with_nonce` | TP | 同类问题：初始化保证完全依赖 `extern` 实现，Rust 侧没有验证其确实完整写入 `MaybeUninit`。 |
| `c::aegis128x2::Aegis128X2Mac::new_with_nonce` | TP | 同类 FFI out-parameter 初始化缺口。 |
| `c::aegis128x4::Aegis128X4Mac::new_with_nonce` | TP | 同类 FFI out-parameter 初始化缺口。 |
| `c::aegis256::Aegis256Mac::new_with_nonce` | TP | 同类 FFI out-parameter 初始化缺口。 |
| `c::aegis256x2::Aegis256X2Mac::new_with_nonce` | TP | 同类 FFI out-parameter 初始化缺口。 |
| `c::aegis256x4::Aegis256X4Mac::new_with_nonce` | TP | 同类 FFI out-parameter 初始化缺口。 |
| `c::raf::scratch::ScratchBuf::new` | TP | safe `new(0)` 生成 zero-sized `Layout` 并传给 `alloc_zeroed`；该 allocator API 要求 layout size 非零，后置 null assert 不能修复已经违反的前置条件。 |

### arrow2-0.18.0

| Function | Result | 独立检查 |
| --- | ---: | --- |
| `compute::sort::row::Rows::row_unchecked` | TP | public safe `row` 没有 bounds check；`row >= self.len()` 时，`offsets.get_unchecked(row + 1)` 或紧随其后的 unchecked access 越界。该模块需要以 `compute_sort` feature 扫描。 |

### bitm-0.4.2

| Function | Result | 独立检查 |
| --- | ---: | --- |
| `rank_select::RankSimple::try_rank` | TP | `content`/`ranks` 可被安全构造成长度关系不一致的值，`ranks.get_unchecked(block)` 可越界。 |
| `rank_select::select::select64` | TP | public safe `rank` 未限制为小于 `n.count_ones()`，可令 `SELECT_U8.get_unchecked(...)` 越界。 |

### blade-0.2.0

| Function | Result | 独立检查 |
| --- | ---: | --- |
| `BufferPiece::data` | TP | `BufferPiece.offset` public，safe `data()` 未检查 offset 是否仍在 mapped buffer allocation 内，直接把它转换为 `isize` 后交给 raw pointer `offset`。 |

### cyberex-0.3.12

| Function | Result | 独立检查 |
| --- | ---: | --- |
| `void::delete` | TP | public safe 函数接受任意 `*mut c_void`，无法保证它来自 layout 匹配的 `Box<T>`，直接违反 `Box::from_raw` 的有效性约束。 |
| `void::opacue_to_ref` | TP | 任意 raw pointer 被 safe 函数直接解引用，没有 null、alignment、可读性或 lifetime 保证。 |
| `xffi::sto::cchar_to_string` | TP | 只检查 null，未保证指针可读且以 NUL 结尾，`CStr::from_ptr` 的具体规则可被违反。 |
| `xffi::xtr::string_to_buffer` | TP | public `buf` 可为不可写或与 `s` 重叠的任意 pointer；RukLee 的 destination-valid/disjoint 报告成立。 |
| `xffi::xtr::string_to_dbuffer` | TP | public `buf`/`buf_max` 可为 null、dangling 或不可写，两个 raw-pointer store 均安全可达地违规。 |

### fast-able-1.11.17

| Function | Result | 独立检查 |
| --- | ---: | --- |
| `vec::SyncVec::get_uncheck` | TP | public safe 方法接受任意 index，直接传给 `get_unchecked`。 |

### fast-float-0.2.0

| Function | Result | 独立检查 |
| --- | ---: | --- |
| `common::AsciiStr::check_len` | TP | safe `check_len(n)` 接受任意 `n`，却在比较结果前执行 `self.ptr.add(n)`；当 `n` 超过原 slice allocation 的 in-bounds/one-past 范围时，`add` 本身已经 UB。 |

### falcon_capstone-0.5.2

| Function | Result | 独立检查 |
| --- | ---: | --- |
| `capstone::InstrBuf::get` | TP | public safe `InstrBuf::new(ptr, count, ...)` 接受任意 raw pointer/count；`get` 只验证 `index < count`，不能证明 `ptr.add(index)` 仍在有效 allocation 内。 |

### file_mmap-0.20.0

| Function | Result | 独立检查 |
| --- | ---: | --- |
| `FileMmap::write` | TP | public safe `write(addr, bytes)` 接受任意 signed `addr`，未验证 mmap bounds；`as_mut_ptr().offset(addr)` 可落到 mapping allocation 外，随后还被用于构造 mutable slice。 |

### grubbnet-0.1.7

| Function | Result | 独立检查 |
| --- | ---: | --- |
| `buffer::NetworkBuffer::drain` | TP | `count` 未检查且 `offset` public；pointer offset 和 copy 的源、目标范围都可被安全输入破坏。 |

### halo2curves-axiom-0.7.0

| Function | Result | 独立检查 |
| --- | ---: | --- |
| `prefetch` | TP | public safe `offset` 未限制为 `<= data.len()`；即使 prefetch 不解引用，`data.as_ptr().add(offset)` 本身也要求 in-bounds/one-past。 |

### hftbacktest-0.4.0

| Function | Result | 独立检查 |
| --- | ---: | --- |
| `backtest::data::DataPtr::at` | TP | public safe `at(index)` 没有检查 `index < self.len()`，直接对 backing slice pointer 调用 `add(index)`。 |

### hugepage-rs-0.1.0

| Function | Result | 独立检查 |
| --- | ---: | --- |
| `alloc` | TP | safe wrapper 接受 zero-sized `Layout`，但 `GlobalAlloc::alloc` 的具体 rule 要求 layout size 非零。 |
| `boxed::Box::new` | TP | allocator 忽略 `layout.align()`，只依赖 mmap/huge-page 的自然对齐；安全调用者可选择 alignment 大于 huge-page alignment 的 `T`，得到 non-null 但未按 `T` 对齐的 pointer，使 `NonNull::as_mut` 的 reference-convertibility rule 失效。 |
| `dealloc` | TP | 函数写成 safe，调用者却可传任意 pointer 或不匹配的 layout，直接违反 `GlobalAlloc::dealloc` 的 ownership/layout rules。 |

### i_tree-0.9.0

| Function | Result | 独立检查 |
| --- | ---: | --- |
| `tree::Tree::mut_node` | TP | public safe node index 未做 bounds check。 |
| `tree::Tree::node` | TP | public safe node index 未做 bounds check。 |

### i_triangle-0.26.0

| Function | Result | 独立检查 |
| --- | ---: | --- |
| `delaunay::delaunay::Delaunay::build` | TP | `Delaunay.triangles`、triangle index 和 neighbors public；crate 内 safe 调用可把越界 neighbor 交给 `get_unchecked`。 |
| `delaunay::triangle::DTriangle::neighbor_by_order` | TP | public safe `order` 未限制为 `0..3`。 |
| `delaunay::triangle::DTriangle::vertex_by_order` | TP | public safe `order` 未限制为 `0..3`。 |
| `monotone::mslice_buffer::MSliceBuffer::add_connections` | TP | safe 方法可重复调用：第一次用长 triangle slice 留下较大 `edge.triangle`，第二次传短 slice 时该旧值未经重验即进入 `get_unchecked_mut`。 |
| `monotone::mslice_buffer::MSliceBuffer::new` | TP | crate 内 safe `MSlice::new(a, b)` 接受任意端点；令端点大于 `vertex_count` 即可使 `vertex_marks.get_unchecked_mut` 越界。 |

### jh-0.1.0

| Function | Result | 独立检查 |
| --- | ---: | --- |
| `compressor::f8_impl` | TP | public safe 函数接受任意 raw `data` pointer；多个 word offset/read 的 allocation、range 和可读性规则均无保证。 |

### jomini-0.27.0

| Function | Result | 独立检查 |
| --- | ---: | --- |
| `buffer::BufferWindow::advance` | TP | `start`/`end` public，且 safe `amt` 任意；`start.add(amt)` 的 allocation/range/no-wrap rules 可直接被破坏。 |
| `buffer::BufferWindow::consumed_data` | TP | public `start` 可与 private `start_buf` 来自不同 allocation，违反 `offset_from` same-allocation rule。 |
| `buffer::BufferWindow::fill_buf` | TP | public BufferWindow 状态可被改坏；此外 safe `Read` 可谎报大于 buffer 的长度，使 `end.add(r)` 越界。 |
| `buffer::BufferWindow::get` | TP | safe public 方法直接接受任意 raw-pointer range，`offset_from` 和 `from_raw_parts` 的同 allocation、顺序、nonnull、长度 rules 均可被破坏。 |
| `buffer::BufferWindow::window` | TP | public `start`/`end` 可被安全设为任意 pointer；RukLee 报出的 `from_raw_parts` pointer rules 确实无保证。 |
| `buffer::BufferWindow::window_len` | TP | public `start`/`end` 可来自不同 allocations，直接违反 `offset_from` rule。 |

### lurk-cli-0.3.8

| Function | Result | 独立检查 |
| --- | ---: | --- |
| `arch::read_string` | TP | public safe API 接受任意整数地址并 cast 为 pointer 后调用 `offset(count)`；没有 allocation provenance 或 in-bounds 保证，循环到非零 `count` 时 RukLee 可确定违反 pointer-offset range rule。 |

### memmap-0.7.0

| Function | Result | 独立检查 |
| --- | ---: | --- |
| `unix::MmapInner::flush` | TP | public safe `MmapMut::flush_range(offset, len)` 不检查文档要求的 range bounds，直接把任意 `offset` 传入；内部 page-alignment 运算后对 mapping pointer 调用 `offset`，安全调用者可令结果落到 mmap allocation 之外。 |

### meshopt-0.3.0

| Function | Result | 独立检查 |
| --- | ---: | --- |
| `optimize::optimize_overdraw_in_place` | TP | `VertexDataAdapter.position_offset` public，可超过 reader slice，RukLee 报出的 pointer-add range rules 成立。 |
| `shadow::generate_shadow_indices` | TP | 同上。 |
| `simplify::simplify` | TP | 同上。 |
| `simplify::simplify_sloppy` | TP | 同上。 |
| `simplify::simplify_with_locks` | TP | 同上。 |
| `utilities::VertexDataAdapter::pos_ptr` | TP | 方法直接把 public `position_offset` 传给 pointer `add`，无需经过 constructor 即可破坏范围。 |

### parsepatch-0.3.1

| Function | Result | 独立检查 |
| --- | ---: | --- |
| `parser::LineReader::parse_numbers` | TP | public safe 方法无条件执行 `self.buf.get_unchecked(4..)`；`buf.len() < 4` 时 RangeFrom 的起点越界。RukLee 的 public-field control chain 可将 `buf.length` 确认为 caller-controllable。 |

### ptr_hash-0.1.1

| Function | Result | 独立检查 |
| --- | ---: | --- |
| `tiny_ef::TinyEf::index` | TP | public safe index 没有检查 `index / 44 < ef.len()`。 |
| `util::prefetch_index` | TP | public safe index 可大于 slice length，pointer `add` 已经违反 range rule。 |

### qwt-0.2.0

| Function | Result | 独立检查 |
| --- | ---: | --- |
| `utils::prefetch_read_NTA` | TP | public safe `offset` 未限制在 slice allocation 内；`wrapping_add` 可产生越界 pointer，随后把它交给 unsafe `_mm_prefetch` 作 read prefetch，违反 pointer read-valid rule。 |

### revm-interpreter-10.0.3

| Function | Result | 独立检查 |
| --- | ---: | --- |
| `instructions::control::rjump` | TP | public instruction API 和 public pointer 可接收未经 EOF verifier 保证的 immediate/jump offset，最终 pointer offset 可跳出 allocation。 |
| `instructions::control::rjumpi` | TP | 条件为真时 signed immediate 可令最终 pointer offset 越界。 |
| `instructions::control::rjumpv` | TP | public unverified jump table可令读取表项或最终 pointer offset 越过 bytecode allocation。 |
| `instructions::stack::push` | TP | public pointer 可无效或不足 const `N` bytes，`from_raw_parts(ip, N)` 及 `ip.add(N)` 的具体 rules 均可违反。 |
| `interpreter::Interpreter::current_opcode` | TP | `instruction_pointer` public，safe 调用者可令 raw dereference 无效。 |
| `interpreter::Interpreter::load_eof_code` | TP | safe `pub(crate)` 接受任意 `pc`；code index 有检查，但 `bytecode.as_ptr().add(pc)` 无 bounds check。 |
| `interpreter::Interpreter::program_counter` | TP | public pointer 可与 `bytecode.as_ptr()` 不属于同一 allocation，违反 `offset_from` rule。 |
| `interpreter::Interpreter::step` | TP | public pointer 可令 opcode raw dereference 无效；该函数至少有这一条 RukLee violation reason 成立。 |
| `interpreter::stack::Stack::dup` | TP | public `data_mut()` 可 shrink capacity；`dup` 只检查 `STACK_LIMIT`，未检查当前 Vec capacity，copy destination/set_len rules 可违反。 |
| `interpreter::stack::Stack::exchange` | TP | release 下 `n + m` 可回绕并通过 guard，随后原始大 `n` 进入 `top.sub(n)`，RukLee 的 sub/swap range rules 成立。 |
| `interpreter::stack::Stack::push_slice` | TP | public `data_mut()` 可破坏固定 capacity 不变量，随后 `set_len(new_len)` 或目标写入可超过实际 allocation。 |

### roqoqo-quest-0.14.4

| Function | Result | 独立检查 |
| --- | ---: | --- |
| `quest_bindings::ComplexMatrixN::set` | TP | `complex_matrix`、`dimension` public；安全代码可伪造 pointer table/dimension，使 row/column pointer add 越界。 |

### rust-crypto-0.2.36

| Function | Result | 独立检查 |
| --- | ---: | --- |
| `cryptoutil::write_u64v_le` | TP | safe 空 slice 输入 `dst = &mut []`、`input = &[]` 通过长度 assert，却立即执行 `dst.get_unchecked_mut(0)`（也对 input 取 unchecked 0），在进入零次循环前已经越界 UB。 |
| `cryptoutil::write_u32v_le` | TP | 同类 empty-slice 反例：`0 == 4 * 0`，随后 `get_unchecked_mut(0)` 越界。 |
| `cryptoutil::read_u64v_be` | TP | 两个空 slice 会使初始 `get_unchecked_mut(0)` 越界；非空路径还调用 `mem::uninitialized::<u64>()`，按被报告的 rule-192 本身就是 immediate UB。 |
| `cryptoutil::read_u64v_le` | TP | 与 `read_u64v_be` 相同，empty-slice unchecked index 和非空路径的 uninitialized integer 均成立。 |
| `cryptoutil::read_u32v_be` | TP | empty-slice 输入安全可达地触发 unchecked index；非空循环还构造 uninitialized `u32`。 |
| `cryptoutil::read_u32v_le` | TP | 与 `read_u32v_be` 相同。 |
| `cryptoutil::read_u32_le` | TP | 长度 4 的合法 safe slice 会到达 `mem::uninitialized::<u32>()`；即使紧接着按字节写满，调用该 API 产生未初始化整数时已经违反 rule-192。 |
| `cryptoutil::read_u32_be` | TP | 与 `read_u32_le` 相同，reported `mem::uninitialized::<u32>` rule 确实成立。 |
| `ghash::Ghash::new` | TP | 合法 16-byte key 会到达 `mem::uninitialized::<[Gf128; 128]>()`；随后循环写满数组不能撤销构造未初始化 typed array 时已经发生的 UB。 |
| `util::fixed_time_eq` | TP | 两个 empty slice 长度相等，会进入 unsafe 分支并对两者执行 `get_unchecked(0)`；count 为 0 只影响后续 FFI 调用，不能使已经发生的 unchecked 越界合法。 |

### shaman-0.1.0

| Function | Result | 独立检查 |
| --- | ---: | --- |
| `cryptoutil::write_u64v_le` | TP | 两个 empty slice 通过长度 assert，随后 `get_unchecked_mut(0)` 在零次循环前越界。 |
| `cryptoutil::write_u32v_le` | TP | 同类 empty-slice unchecked-index UB。 |
| `cryptoutil::read_u64v_be` | TP | empty slice 可触发初始 unchecked index；非空路径的 `mem::uninitialized::<u64>()` 也独立违反 rule-192。 |
| `cryptoutil::read_u64v_le` | TP | 与 `read_u64v_be` 相同。 |
| `cryptoutil::read_u32v_be` | TP | empty slice 可触发初始 unchecked index；非空路径还构造 uninitialized `u32`。 |
| `cryptoutil::read_u32v_le` | TP | 与 `read_u32v_be` 相同。 |
| `cryptoutil::read_u32_le` | TP | 合法 4-byte input 会到达 reported `mem::uninitialized::<u32>()`，属于 immediate UB。 |
| `cryptoutil::read_u32_be` | TP | 与 `read_u32_le` 相同。 |

### splay_tree-0.3.1

| Function | Result | 独立检查 |
| --- | ---: | --- |
| `tree_core::Tree::node_mut` | TP | crate 内可创建空 Tree 后直接调用 safe public `node_mut(0)`。 |
| `tree_core::Tree::node_ref` | TP | crate 内可创建空 Tree 后直接调用 safe public `node_ref(0)`。 |

### stackvector-1.1.1

| Function | Result | 独立检查 |
| --- | ---: | --- |
| `StackVec::dedup_by` | TP | `StackVec.length` public，可大于 backing array capacity，使循环中的 pointer offsets 越界。 |
| `StackVec::drain` | TP | public `length` 可大于 capacity，使 `from_raw_parts_mut` 的 validity/initialization rules 失效。 |
| `StackVec::from_vec` | TP | safe API 对任意 `A::Item` 使用 `mem::uninitialized::<A>()`；例如包含 `bool`/reference 的 array 不能处于未初始化状态，RukLee 的 uninitialized rule 成立。 |
| `StackVec::insert` | TP | public `length` 可伪造为超出 backing array 的值，使 reported offset/copy destination rules 失效。 |
| `StackVec::insert_from_slice` | TP | public `length` 可在 release 中令长度加法回绕并通过 assert，内部 shift/copy 的 destination validity rules 可违反；虽然借用仍保证最终 nonoverlap，但函数还有正确的 RukLee rule。 |
| `StackVec::insert_many` | TP | public `length` 可破坏 old length/capacity 与实际 backing storage 的关系，使 pointer add/copy rules 失效。 |
| `StackVec::into_inner` | TP | safe 代码可把 public `length` 设为 `A::size()`，但 `data` 仍未初始化；随后 `ptr::read` 违反 initialized/valid rules。 |
| `StackVec::pop` | TP | public `length` 可设为非零而 data 未初始化，`ptr::read` 可越界或读取未初始化元素。 |
| `StackVec::remove` | TP | public `length` 可伪造，reported read/offset/copy validity rules 可被安全状态破坏。 |
| `StackVec::truncate` | TP | public `length` 可大于 capacity或指向未初始化元素，`drop_in_place` 的 alignment/validity/read-write rules 可违反。 |

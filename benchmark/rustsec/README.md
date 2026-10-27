# RustSec unsafe API benchmark

## 批量运行 RuKLEE

在仓库根目录运行（Python 3.11+，使用本机已有的 Rust/KLEE 构建）：

```bash
python benchmark/rustsec/run.py
```

自动发现本目录下含 `Cargo.toml` 的 crate，串行运行。默认只验证
`run.py` 中 `ADVISORY_TARGETS` 硬编码的 RustSec 问题调用点和规则，
不再遍历整个 crate。每项旁边标有公告编号和问题函数；已有案例沿用
所选缺陷的代表调用点，辅助函数中的实际危险操作也可列入清单。
例如 `iced-x86` 只跑 `Decoder::new` 的 `get_unchecked(len)` / `rule-575`，
从原来的 297 项缩减到 1 项；`bam` 只跑 `Block::load` 的 `set_len` / `rule-961`。
新增 crate 必须先补清单，缺少配置会报错，不会回退到全量运行。

第三批补充（`supplement3-manifest.json`，由 `dev/prepare-rustsec-supplement3.py` 归档）：
`simple-slab 0.3.2`（RUSTSEC-2020-0039，`Slab::remove` 的 `offset(len)`）、
`toodee 0.2.4`（RUSTSEC-2021-0028，`insert_row`）、
`ordnung 0.0.1`（RUSTSEC-2020-0038，`compact::Vec::push`）。三者的公告操作都是 std/core unsafe 调用，
选入前已确认 initial 阶段对该调用点报 SAT（`violation`/`violation-timeout`，带 certain symbol）。
泛型函数靠 `run.py` 中 `INSTANTIATIONS` 的最小实例化 `#[test]`，只写入 `.local/rustsec-work/` 的副本并自动启用 `--test`，
benchmark 源码保持原样。按 `ADVISORY_TARGETS` 统计，initial 阶段 SAT 的 crate 共 16 个：
12 个 `violation`（anon-vec、binary_vec_io、fast-able、fast_id_map、iced-x86、lzf、ordnung、ruint、shaman、simple-slab、smallvec、toodee），
另有 4 个 `low-confidence-sat`（orx-pinned-vec、scanner、tanton_engine、xmas-elf）。
`--crate-timeout 180` 下 ordnung、toodee 的 PoC 阶段会被 3 分钟总时限截断，initial 矩阵已完整写出；simple-slab 的 `Slab::remove` 读越界已被 PoC 回放确认。

新结果写入
`eval/rustsec20/<crate>/ruklee/batch-<时间>/`，包括 `run.json`、
`matrix.json`、`stdout.log`、`stderr.log`、`logs/` 和 `artifacts/`。
`selection.json`、`callsites.txt` 和 `rule-dsl.json` 保存本次选择与原 DSL 的子集，
不会修改共享的 `ptr_rule_dsl.json`。成功退出后还会核对矩阵是否恰好完成全部指定项。
源码先复制到 `.local/rustsec-work/`，不会在 benchmark 原始源码中注入测试。

默认检查 `eval/rustsec20`、`eval/rustsec`、`eval/rustsec-supplement` 和
`eval/rustsec-supplement2` 的历史结果：进程成功退出、矩阵完成且覆盖当前全部
target/rule 对的 crate 会跳过。
这包括旧的定向验证、仅符号执行及 `not-in-ir` 结果；“跳过”仅表示跑过，
不表示检出公告或覆盖全部调用。失败、中断或缺少完整矩阵的尝试会重跑。
新运行启用工具的 PoC 生成与回放。`bam`、`partial_sort`、`ruint`、`transpose`
会自动使用 `--no-debug-assertions`，保留公告所需的 release 行为；这些配置只复用
同样关闭了 debug assertions 的历史运行。
默认仅编译 library；泛型未实例化时可能返回 `not-in-ir`，可用 `--tests`
编译现有单元测试以增加实例化覆盖。此入口不自动添加类型实例化 harness。
若提取器没有产出指定的调用点，运行会失败，不会改跑无关函数。
`maligned`、`safe-transmute` 缺少对应的 `Vec::from_raw_parts` DSL，明确标为
`UNSUPPORTED` 并跳过。`direct_ring_buffer` 的 `rule-961` 只检查容量，
尚不能表达公告中的初始化条件；跑完该规则不等于检出公告。

```bash
python benchmark/rustsec/run.py --dry-run           # 查看决策及具体 target/rule 对
python benchmark/rustsec/run.py --rerun             # 重跑公告目标，保留旧结果
python benchmark/rustsec/run.py --crate iced-x86    # 只跑该 crate 的公告目标
python benchmark/rustsec/run.py --crate ruint --rerun --no-debug-assertions
python benchmark/rustsec/run.py --tests --skip-llm  # 编译现有测试，只跑符号阶段
python benchmark/rustsec/run.py --advisory-only     # 兼容旧参数；现在默认始终定向运行
```

`--timeout` 默认为每次 KLEE 调用 60 秒；`--crate-timeout` 默认为 300 秒，
整个 crate 的验证流程（含 PoC 生成与回放）超过 5 分钟就终止、记录 `timeout`
并继续下一个。可显式传 `--crate-timeout 0` 取消总时限。
每个 crate 失败后继续下一个；有失败或超时时脚本退出码为 1。
Ctrl-C 会终止当前子进程组并保留日志。修改运行选项后，如需重新验证已完成的
crate，应同时指定 `--rerun`。不要与旧的 `dev/run-rustsec.py` 同时运行，
因为工具共享 metadata 和插桩目录。

## 20 案例清单（16 检出 + 4 未检出）

前 16 个在 initial 阶段对公告的 std/core unsafe 调用报 SAT（调用点和规则见 `run.py` 的 `ADVISORY_TARGETS`）；
后 4 个是 RuKLEE 目前检不到、且原因明确的案例（均保留在 `ADVISORY_TARGETS`/`UNSUPPORTED` 中）。
目录里其他 crate（bam、byte_struct、crypto2、elf_rs、partial_sort、safe-transmute、slab、transpose 等）不属于这 20 个。

| # | crate | advisory | unsafe 操作 | initial 结果 |
|---|---|---|---|---|
| 1 | anon-vec 0.1.1 | RUSTSEC-2025-0039 | `AnonVec::get_ref` 指针算术 | violation |
| 2 | binary_vec_io 0.1.12 | RUSTSEC-2025-0109 | `from_raw_parts` / 单个引用构造超长 slice | violation |
| 3 | fast-able 1.11.17 | RUSTSEC-2025-0063 | `SyncVec::get_uncheck` 的 `get_unchecked` | violation |
| 4 | fast_id_map 0.1.0 | RUSTSEC-2025-0034 | `FastMap::get` 的 `get_unchecked` | violation |
| 5 | iced-x86 1.10.3 | RUSTSEC-2021-0068 | `Decoder::new` 的 `get_unchecked(len)` | violation |
| 6 | lzf 0.3.1 | RUSTSEC-2022-0067 | `compress` 的 `uninitialized` 数组 | violation |
| 7 | ordnung 0.0.1 | RUSTSEC-2020-0038 | `compact::Vec::push` 的 `copy_nonoverlapping` | violation-timeout |
| 8 | ruint 1.16.0 | RUSTSEC-2025-0137 | `reciprocal_mg10` 的 `get_unchecked` | violation |
| 9 | shaman 0.1.0 | RUSTSEC-2025-0113 | `write_u64v_le` 的 `get_unchecked_mut` | violation |
| 10 | simple-slab 0.3.2 | RUSTSEC-2020-0039 | `Slab::remove` 的 `offset(len)` / `ptr::read` | violation |
| 11 | smallvec 0.6.12 | RUSTSEC-2018-0018 | `SmallVec::new` 的 `uninitialized` | violation |
| 12 | toodee 0.2.4 | RUSTSEC-2021-0028 | `insert_row` 的 `ptr.add` / `copy` | violation |
| 13 | orx-pinned-vec 3.20.0 | RUSTSEC-2025-0106 | `index_of_ptr` 指针算术 | low-confidence-sat |
| 14 | scanner 0.1.0 | RUSTSEC-2025-0033 | `Match::ptr` 指针算术 | low-confidence-sat |
| 15 | tanton_engine 1.0.0 | RUSTSEC-2025-0031 | `ThreadStack::get` 的 `get_unchecked` | low-confidence-sat |
| 16 | xmas-elf 0.9.1 | RUSTSEC-2025-0018 | `HashTable::get_bucket` 指针算术 | low-confidence-sat |
| 17 | maligned 0.2.1 | RUSTSEC-2023-0017 | `Vec::from_raw_parts` 分配/释放布局不一致 | 未检出 |
| 18 | direct_ring_buffer 0.2.1 | RUSTSEC-2025-0105 | `Vec::set_len` 暴露未初始化元素 | 未检出（verified） |
| 19 | linked-hash-map 0.5.2 | RUSTSEC-2020-0026 | `into_iter` 中 `uninitialized(HashMap)` | 未检出（无调用点） |
| 20 | borrowck_sacrifices 0.1.0 | RUSTSEC-2025-0107 | `from_raw_parts` 读到 padding 未初始化字节 | 未检出（not-in-ir / verified） |

13–16 是 `low-confidence-sat`：DSL 有解，但反例依赖的符号不是调用者可控的，是否可构造 PoC 需要人工确认。

### 4 个未检出的原因

1. **maligned：缺少对应的 DSL 规则。** 缺陷是 `Vec::from_raw_parts` 用的 layout 与分配时不一致。MIRScan 提取到了该调用，但冻结的
   `ptr_rule_dsl.json` 中没有 `alloc::vec::Vec::from_raw_parts` 的规则（`from_raw_parts` 相关规则只有 slice 和 str 版本），
   所以没有可检查的契约，`run.py` 将其标为 `UNSUPPORTED`。
2. **direct_ring_buffer：规则只覆盖契约的一部分。** 缺陷是 `set_len` 把未初始化的元素暴露出来。现有 `set_len` 规则（rule-961）
   只检查 `new_len <= capacity`，没有“`0..new_len` 已初始化”这一半契约。实例化后 rule-961 报 `verified`，这是规则本身的结论，不是漏扫。
3. **linked-hash-map：提取器没有产出调用点。** 真正的缺陷是 `IntoIterator::into_iter` 这个 trait 实现里的
   `uninitialized::<HashMap>()`（`src/lib.rs:1137`）。MIRScan 没有提取 trait 实现内部的调用，
   `x verify` 报 `no callsites matched`，根本没进入符号执行。同样的 rule-192 在 lzf、smallvec 上都能检出，所以问题不在规则。
4. **borrowck_sacrifices：缺陷依赖具体类型实例化。** `any_as_u8_slice<T>` 只有当 `T` 含 padding 时才暴露未初始化字节。
   仅编译 library 时泛型函数不在 LLVM IR 中（`not-in-ir`）；用 `u64` 这类无 padding 的类型实例化后 rule-611 报 `verified`。
   检出需要为公告选一个带 padding 的具体类型，这个选择取决于缺陷知识，工具不能自动推断。

## 原有十案例实验

10 个不同 crate 的 10 个已知缺陷；RustSec 网页的 **Issued < 2026-03-01**，且下载版本的发布时间也早于该日期。选择覆盖 bounds、initialization、allocation layout、ownership 和 panic safety，属于目的性抽样，不能据此推断整个 RustSec 的召回率。

`manifest.json` 记录公告 URL、Issued/Reported、受影响版本和 crates.io SHA-256；`cases.json` 固定每个公告的具体缺陷、源码位置和独立反例推理。`evidence/` 保存 RustSec 原文、HTML、原始 `.crate` 归档和必要的依赖锁文件。源码直接来自 crates.io，没有修复缺陷。兼容配置、插桩和实例化只用于 `.local/` 或 `crates_inj/` 中的实验副本。

| crate | version | advisory | unsafe API / 缺陷 |
|---|---|---|---|
| binary_vec_io | 0.1.12 | [2025-0109](https://rustsec.org/advisories/RUSTSEC-2025-0109.html) | from_raw_parts / 单个引用构造超长 slice |
| borrowck_sacrifices | 0.1.0 | [2025-0107](https://rustsec.org/advisories/RUSTSEC-2025-0107.html) | from_raw_parts / padding 未初始化 |
| direct_ring_buffer | 0.2.1 | [2025-0105](https://rustsec.org/advisories/RUSTSEC-2025-0105.html) | set_len / 未初始化元素 |
| maligned | 0.2.1 | [2023-0017](https://rustsec.org/advisories/RUSTSEC-2023-0017.html) | Vec::from_raw_parts / 分配释放布局不匹配 |
| safe-transmute | 0.10.0 | [2018-0013](https://rustsec.org/advisories/RUSTSEC-2018-0013.html) | Vec::from_raw_parts / len、capacity 颠倒 |
| byte_struct | 0.6.0 | [2021-0032](https://rustsec.org/advisories/RUSTSEC-2021-0032.html) | uninitialized / panic 时 drop 无效对象 |
| elf_rs | 0.2.0 | [2022-0079](https://rustsec.org/advisories/RUSTSEC-2022-0079.html) | add、from_raw_parts / ELF offset 未检查 |
| smallvec | 0.6.12 | [2018-0018](https://rustsec.org/advisories/RUSTSEC-2018-0018.html) | uninitialized / 任意泛型类型 |
| linked-hash-map | 0.5.2 | [2020-0026](https://rustsec.org/advisories/RUSTSEC-2020-0026.html) | uninitialized / HashMap 内部 NonNull 无效值 |
| lzf | 0.3.1 | [2022-0067](https://rustsec.org/advisories/RUSTSEC-2022-0067.html) | uninitialized / usize 数组 |

公告编号年份、Reported 和 Issued 并不相同。例如两个 2018 编号的公告网页 Issued 为 2020-10-01。本实验使用 Issued 判断是否满足截止日期。

运行和结果在仓库根目录的 `dev/prepare-rustsec.py`、`dev/run-rustsec.py` 和 `eval/rustsec/`。baseline runner 复用 `docker/rust-comparison/run.py`，新增可配置 benchmark/output root，原有默认路径保持不变。

```bash
python3 dev/prepare-rustsec.py
python3 dev/run-rustsec.py baselines --attempt my-run --timeout 180
python3 dev/run-rustsec.py safe4u --attempt my-run --timeout 3600
python3 dev/run-rustsec.py ruklee --attempt my-run --timeout 60
```

本机 Rust nightly 从已有 devcontainer 镜像导出到 `.local/rustsec-toolchain/`。Safe4U 使用已有编译好的 context retriever，通过 `.local/rustsec-bin/cargo` 转发运行；该 shim 对其他 Cargo 命令透明。Safe4U 使用 `gpt-4o-mini` 和原有 `Safe4U` prompt，沿用工具的 decomposition cache。详细配置、工具 hash 和命令见 `eval/rustsec/environment.json` 与逐次 `run.json`。

`--instantiate` 是独立的辅助实验：只在隔离副本添加最小安全 API 调用，以使泛型函数进入 LLVM IR，并编译已有 library tests；不执行这些单元测试。辅助结果必须与未添加调用的结果分开报告。`targets/` 只选择公告对应的危险操作；本工具执行的是定向验证，其他 baseline 通常执行整个 library，MirChecker 按公告设置入口。

统计单位是公告缺陷，不能把无关告警、仅提示审计 unsafe 的规则、编译器 deprecated/invalid_value lint、工具崩溃、找不到入口、缺少 DSL 或没有 LLVM 函数体记作检出。所有失败尝试均保留；依赖固定和兼容重试不修改原始 `.rs`。

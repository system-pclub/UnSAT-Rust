## Directory
- ./crates: A list of real world rust crates
- ./meta:   A list of metadata for each crate in ./crates
- ./tools:  Helper tools for using this dataset

## Setup
```bash
# Initialize submodule 
$ git submodule update --init --recursive
$ ./build_klee.sh
$ ./build_rust.sh
$ ./build_mirscan.sh
$ cd tools/x && uv sync && cd ../..
```

## How to label a crate
```bash
# Step 0: Copy crate folder under ./crates folder with folder name <crate-name>-<version number>
# Step 1: Run following command to generate placeholder files:
$ ./x sync --cargo-dir <crate folder>
# This command will:
# - Automatically find all unsafe APIs under public safe functions
# - Generate placeholder files for rules (from rules.csv and studied_rules) regarding to these unsafe API callsites

# Step 2: fill the <placeholder> for task1/2/3 
```



## How to extract comment
```bash
$ ./build_extractcomment.sh
$ tools/extract-comment/target/release/extract-comment --dirs tools/rust/library/core --out .local/core-comments.json
$ tools/extract-comment/target/release/extract-comment --dirs tools/rust/library/std --out .local/std-comments.json
```

rule-446,rust/library/core/src/ptr/mut_ptr.rs:971,const unsafe fn add,"If the computed offset is non-zero, `self` must be derived from a pointer to some allocated object."
rule-447,rust/library/core/src/ptr/mut_ptr.rs:971,const unsafe fn add,"If the computed offset is non-zero, the entire memory range between `self` and the result must be in bounds of that allocated object."
rule-448,rust/library/core/src/ptr/mut_ptr.rs:971,const unsafe fn add,"If the computed offset is non-zero, the range between `self` and the result must not wrap around the edge of the address space."
rule-449,rust/library/core/src/ptr/mut_ptr.rs:971,

## How to verify a rule
```bash
$ ./x sync --cargo-dir  crates/bevy_hecs-0.3.0

$ ./x verify crates/bevy_hecs-0.3.0 \
  --callsite src-world-rs-408-18 \
  --rule rule-447 \
  --report-json crates_inj/bevy_hecs-0.3.0/report.json \
  --test \
  --compose-loop-bound 1 \
  --rustc "$PWD/tools/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" \
  --klee-bin "$PWD/tools/klee/build/bin/klee"
```


tasks context:

bitm
RankSimple's try_rank's get_unchecked is buggy

jh
compressor.rs's f8_impl's read_unaligned is buggy

tract-core
ZoneScanner's next's get_unchecked_mut is buggy

revm
Interpreter's program_counter's offset_from is buggy
Interpreter's current_opcode's dereferance is buggy


i_triangle
i_triangle::delaunay::triangle::DTriangle::neighbor_by_order and
i_triangle::delaunay::triangle::DTriangle::vertex_by_order 's get_unchecked is buggy

aegis
ScratchBuf's new's alloc_zeroed's is buggy


jomini
BufferWindow'sfunctions advance, window, window_len, consumed_data, get and fill_buf 's unsafe callees are buggy

i_tree
`Tree::node` and `Tree::mut_node` methods in the public `tree` module. These methods accepted an arbitrary `u32` index and passed it directly to `Vec::get_unchecked` / `get_unchecked_mut` 
- src-tree-rs-65-13
- src-tree-rs-72-13

splay_tree
src/tree_core.rs, node_ref and node_mut's get_unchecked and get_unchecked_mut are buggy

hugepage-rs
hugepage-rs/src/lib.rs's dealloc is buggy

Above are confirmed buggy place

Jobs:
1. make sure corresponding callsite's unsfe callee's rules(which callee have which rules can be checked in rules.csv) exist in ptr_rule_dsl.json (yes you can put any rules into it, not only the ptr rules)
- before doing that, make sure operators can describe the rules
- if no enough operators, add minimal but not duplicated operators (klee also need to add corresponding implementation)

2. run ./x verify (skip llm and rerun) for these crates can actually
- reach these callsites
- result is at least one of unsafe callee's dsl rule need to sat and at least one of variable is certain symbol

first analyze what progress now (since we already run ./x verify for these crates previously)
then figure out what need to do to finish these two jobs


bitm
src-rank_select-mod-rs-328-21，try_rank -> get_unchecked
src-rank_select-mod-rs-330-25，try_rank -> get_unchecked
jh
src-compressor-rs-112-16
src-compressor-rs-113-16
src-compressor-rs-114-16
src-compressor-rs-115-16
src-compressor-rs-135-16
src-compressor-rs-136-16
src-compressor-rs-137-16
src-compressor-rs-138-16
这些都是 compressor::f8_impl -> core::ptr::read_unaligned。
tract-core
ZoneScanner::next -> get_unchecked_mut：
src-ops-cnn-patches-rs-470-14
src-ops-cnn-patches-rs-476-18
src-ops-cnn-patches-rs-424-14
src-ops-cnn-patches-rs-425-17
src-ops-cnn-patches-rs-431-14
revm
src-interpreter-rs-311-18，Interpreter::current_opcode -> raw ptr deref
src-interpreter-rs-343-18，Interpreter::program_counter -> offset_from
i_triangle
src-delaunay-triangle-rs-13-14，DTriangle::neighbor_by_order -> get_unchecked
src-delaunay-triangle-rs-19-14，DTriangle::vertex_by_order -> get_unchecked
aegis 0.9.12
src-c-raf-scratch-rs-15-28，ScratchBuf::new -> alloc_zeroed
jomini
BufferWindow 相关：
src-buffer-rs-45-28，advance -> ptr.add
src-buffer-rs-52-18，window -> from_raw_parts
src-buffer-rs-57-18，window_len -> offset_from
src-buffer-rs-67-18，consumed_data -> offset_from
src-buffer-rs-74-28，get -> offset_from
src-buffer-rs-75-27，get -> from_raw_parts
src-buffer-rs-100-29，fill_buf -> ptr.add
src-buffer-rs-106-37，fill_buf -> ptr.add
raw ptr deref in fill_buf: src-buffer-rs-86-26, 92-30, 95-13, 99-22, 100-29, 103-32, 105-30
i_tree
src-tree-rs-65-13，Tree::node -> get_unchecked
src-tree-rs-72-13，Tree::mut_node -> get_unchecked_mut
splay_tree
src-tree_core-rs-422-18，node_ref -> get_unchecked
src-tree_core-rs-425-18，node_mut -> get_unchecked_mut
hugepage-rs
src-lib-rs-70-9，dealloc -> GlobalAlloc::dealloc

## Newly discovered candidate bugs from full rerun

Source results:
- `.local/verify/full-10crates-after-fixes/*-full-matrix-results.json`

Only entries with `status=violation` and `full_rerun_passed=true` are listed here.
Entries already listed in the confirmed buggy places above are excluded.

jh
- `src-compressor-rs-113-36`, `compressor::f8_impl -> core::ptr::const_ptr::<impl *const T>::offset`
  - rules: `rule-288`, `rule-289`, `rule-290`, `rule-291`
  - json: `.local/verify/full-10crates-after-fixes/src-compressor-rs-113-36__rule-288.json`, `.local/verify/full-10crates-after-fixes/src-compressor-rs-113-36__rule-289.json`, `.local/verify/full-10crates-after-fixes/src-compressor-rs-113-36__rule-290.json`, `.local/verify/full-10crates-after-fixes/src-compressor-rs-113-36__rule-291.json`
- `src-compressor-rs-114-36`, `compressor::f8_impl -> core::ptr::const_ptr::<impl *const T>::offset`
  - rules: `rule-288`, `rule-289`, `rule-290`, `rule-291`
  - json: `.local/verify/full-10crates-after-fixes/src-compressor-rs-114-36__rule-288.json`, `.local/verify/full-10crates-after-fixes/src-compressor-rs-114-36__rule-289.json`, `.local/verify/full-10crates-after-fixes/src-compressor-rs-114-36__rule-290.json`, `.local/verify/full-10crates-after-fixes/src-compressor-rs-114-36__rule-291.json`
- `src-compressor-rs-115-36`, `compressor::f8_impl -> core::ptr::const_ptr::<impl *const T>::offset`
  - rules: `rule-288`, `rule-289`, `rule-290`, `rule-291`
  - json: `.local/verify/full-10crates-after-fixes/src-compressor-rs-115-36__rule-288.json`, `.local/verify/full-10crates-after-fixes/src-compressor-rs-115-36__rule-289.json`, `.local/verify/full-10crates-after-fixes/src-compressor-rs-115-36__rule-290.json`, `.local/verify/full-10crates-after-fixes/src-compressor-rs-115-36__rule-291.json`

revm
- `src-instructions-control-rs-2-24`, `instructions::control::rjumpv -> std::slice::from_raw_parts`
  - rules: `rule-611`
  - json: `.local/verify/full-10crates-after-fixes/src-instructions-control-rs-2-24__rule-611.json`
- `src-instructions-stack-rs-32-30`, `instructions::stack::push -> std::slice::from_raw_parts`
  - rules: `rule-611`
  - json: `.local/verify/full-10crates-after-fixes/src-instructions-stack-rs-32-30__rule-611.json`
- `src-interpreter-analysis-rs-2-24`, `interpreter::analysis::validate_eof_code -> std::slice::from_raw_parts`
  - rules: `rule-611`
  - json: `.local/verify/full-10crates-after-fixes/src-interpreter-analysis-rs-2-24__rule-611.json`
- `src-interpreter-stack-rs-241-27`, `interpreter::stack::Stack::dup -> std::ptr::mut_ptr::<impl *mut T>::add`
  - rules: `rule-448`, `rule-449`
  - json: `.local/verify/full-10crates-after-fixes/src-interpreter-stack-rs-241-27__rule-448.json`, `.local/verify/full-10crates-after-fixes/src-interpreter-stack-rs-241-27__rule-449.json`
- `src-interpreter-stack-rs-282-23`, `interpreter::stack::Stack::exchange -> std::ptr::mut_ptr::<impl *mut T>::add`
  - rules: `rule-448`, `rule-449`
  - json: `.local/verify/full-10crates-after-fixes/src-interpreter-stack-rs-282-23__rule-448.json`, `.local/verify/full-10crates-after-fixes/src-interpreter-stack-rs-282-23__rule-449.json`
- `src-interpreter-stack-rs-304-23`, `interpreter::stack::Stack::push_slice -> std::ptr::mut_ptr::<impl *mut T>::add`
  - rules: `rule-448`, `rule-449`
  - json: `.local/verify/full-10crates-after-fixes/src-interpreter-stack-rs-304-23__rule-448.json`, `.local/verify/full-10crates-after-fixes/src-interpreter-stack-rs-304-23__rule-449.json`
- `src-opcode-eof_printer-rs-2-24`, `opcode::eof_printer::print_eof_code -> std::slice::from_raw_parts`
  - rules: `rule-611`
  - json: `.local/verify/full-10crates-after-fixes/src-opcode-eof_printer-rs-2-24__rule-611.json`

KLEE modification example
- Example: `revm`, `src-instructions-control-rs-2-24__rule-611`.
  - json: `.local/verify/full-10crates-after-fixes/src-instructions-control-rs-2-24__rule-611.json`
  - init stage: `init_status=violation`, `has_certain_symbol=true`
  - rule: `std::slice::from_raw_parts`, `is_initialized(memory_range(get_arg(0), get_arg(1) * sizeof(T)))`
  - why this demonstrates the KLEE-side improvement: the bug is already detected in the init/KLEE phase, before relying on LLM testcase quality. This depends on KLEE being able to evaluate the new memory-range initialization DSL against a concrete/certain pointer-length pair at the unsafe callsite. The LLM/rerun phase then confirms it with `rerun_reported_callsite=true`, `rerun_dsl_sat=true`, and `full_rerun_passed=true`.


+ find example, not only constructor, but also other methold call like mutable reference, etc..


ruklee used rust features:
+ template
+ struct visbility
+ func(&a, &b) original , dest does not overlap
copy_nonoverlap
+ xxxx (find more case and examples)

+ each feature find an example to prove it works

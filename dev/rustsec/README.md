# RustSec unsafe API benchmark

The corpus is restricted to [allowlist.json](allowlist.json): **38 advisory records, 37 distinct crate versions**. The maintainer confirmed this exact table; `toodee-0.5.0` covers two advisories. `manifest.json` records source availability and exact-version provenance; `cases.json` records the selected defects. No other top-level crate packages belong in `benchmark/rustsec/`.

## Environment and maintenance

Use **Rust 1.87.0**. `dev/rustsec/rust-toolchain.toml` pins isolated work copies; use `cargo +1.87.0` for direct Cargo commands against source snapshots. RuKLEE uses the project's patched compiler, whose actual release must also be checked as `1.87.0` inside Docker. The image's bootstrap nightly is not the benchmark compiler requirement. Keep `benchmark/rustsec/` restricted to crate directories. This `dev/rustsec/` directory contains the runner, tests, metadata, evidence and target lists.

Run all builds and validation in the root-Dockerfile image `unsat-rust:local`, following the [environment skill](../../.agents/skills/ruklee-environment/SKILL.md).

Download a missing pinned source from the repository root:

```bash
python3 tools/crates.py download-one <crate> <version> --output-dir benchmark/rustsec
```

All 37 selected crate versions are present. The maintainer corrected the lexical entry to `lexical-util-0.8.5`, matching the original [Bytes::read report](https://github.com/Alexhuszagh/rust-lexical/issues/102). RUSTSEC-2023-0086 names `lexical-core`; this corpus selects its affected implementation dependency `lexical-util` explicitly.

## RuKLEE batch runner

```bash
python3 dev/rustsec/run.py --dry-run
python3 dev/rustsec/run.py --crate iced-x86
```

For actual runs launch this orchestration script from the host after building the image; it invokes `./x verify` in Docker, serializes access to shared experiment artifacts, and copies each source into `.local/rustsec-work/`. It also carries the Rust toolchain pin into the copy. Results go to `eval/rustsec20/<crate>/ruklee/batch-<timestamp>/`.

All 37 source packages are present. The eight newly configured packages (arrow2, dahl-salso, i_tree, i_triangle, lexical-util, rustdx, toodee and xous) were compiled with the patched Rust 1.87.0 compiler and their selected callsites checked against fresh MIRScan reports. [selection-evidence.json](selection-evidence.json) retains the caller/callee locations and source hashes; compatible dependency locks are stored under `evidence/` and applied only to experiment copies. arrow2 requires `compute_sort`; lexical-util requires `format,parse,parse-integers`.

The full batch dry-run passes: **35 configured package selections and two explicit contract limitations**. `smallvec-0.6.9` needs Vec::from_raw_parts ownership/allocation-layout coverage that the current catalog does not contain. `tremor-script-0.11.5` needs the intrinsic transmute aliasing/lifetime contract; the catalog's transmutability-trait transmute rules describe a different API. These remain `UNSUPPORTED`, without substituting unrelated rules. The undefined rule-963/965/966 references have been removed; reorder now selects the raw-slice initialization contract at the actual from_raw_parts_mut call.

Both toodee advisories are selected: insert_row and the copy operations in DrainCol's nested DropGuard destructor. MIRScan now recognizes safe trait-implementation methods as analysis roots, including nested Drop implementations; a regression test covers this general behavior. The wrapper forwards the extractor's helper/depth options and supplies its patched sysroot.

Compilation, extraction and dry-run results establish configuration coverage, **not vulnerability detection or PoC reproduction**. Full KLEE/PoC evaluation has not been rerun for this preparation change.

Run the reusable corpus check inside Docker:

```bash
docker run --rm -e PYTHONDONTWRITEBYTECODE=1 -v "$PWD:/unsat-rust" -w /unsat-rust unsat-rust:local \
  python3 .agents/skills/rustsec-benchmark/scripts/check_corpus.py --check-selections \
  --rustc tools/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc
```

## Approved cases

| Crate version | Function | Unsafe API | Advisory |
|---|---|---|---|
| `transpose-0.2.2` | `out_of_place::transpose` | `core::slice::get_unchecked / core::slice::get_unchecked_mut` | [RUSTSEC-2023-0080](https://rustsec.org/advisories/RUSTSEC-2023-0080.html) |
| `binary_vec_io-0.1.12` | `binary_write_from_ref / binary_read_to_ref` | `std::slice::from_raw_parts / std::slice::from_raw_parts_mut` | [RUSTSEC-2025-0109](https://rustsec.org/advisories/RUSTSEC-2025-0109.html) |
| `ruint-1.17.0` | `algorithms::div::reciprocal::reciprocal_mg10` | `core::slice::get_unchecked` | [RUSTSEC-2025-0137](https://rustsec.org/advisories/RUSTSEC-2025-0137.html) |
| `fast-able-1.11.17` | `vec::SyncVec::get_uncheck` | `core::slice::get_unchecked` | [RUSTSEC-2025-0063](https://rustsec.org/advisories/RUSTSEC-2025-0063.html) |
| `shaman-0.1.0` | `cryptoutil::write_u64v_le` | `core::slice::get_unchecked_mut / core::slice::get_unchecked` | [RUSTSEC-2025-0113](https://rustsec.org/advisories/RUSTSEC-2025-0113.html) |
| `arrow2-0.18.0` | `compute::sort::row::Rows::row_unchecked` | `core::slice::get_unchecked` | [RUSTSEC-2025-0038](https://rustsec.org/advisories/RUSTSEC-2025-0038.html) |
| `xous-0.9.50` | `definitions::MemoryRange::as_slice / as_slice_mut` | `std::slice::from_raw_parts / std::slice::from_raw_parts_mut` | [RUSTSEC-2024-0431](https://rustsec.org/advisories/RUSTSEC-2024-0431.html) |
| `simple-slab-0.3.2` | `Slab::remove` | `std::ptr::mut_ptr::offset` | [RUSTSEC-2020-0039](https://rustsec.org/advisories/RUSTSEC-2020-0039.html) |
| `i_tree-0.9.0` | `tree::Tree::node / tree::Tree::mut_node` | `core::slice::get_unchecked / core::slice::get_unchecked_mut` | [RUSTSEC-2025-0165](https://rustsec.org/advisories/RUSTSEC-2025-0165.html) |
| `i_triangle-0.26.0` | `delaunay::triangle::DTriangle::neighbor_by_order / vertex_by_order` | `core::slice::get_unchecked` | [RUSTSEC-2025-0164](https://rustsec.org/advisories/RUSTSEC-2025-0164.html) |
| `rustdx-0.4.3` | `bytes_helper::into_arr2 / bytes_helper::u8_from_le_bytes` | `core::slice::get_unchecked` | [RUSTSEC-2026-0123](https://rustsec.org/advisories/RUSTSEC-2026-0123.html) |
| `dahl-salso-0.6.6` | `clustering::Clusterings::label` | `core::slice::get_unchecked` | [RUSTSEC-2026-0129](https://rustsec.org/advisories/RUSTSEC-2026-0129.html) |
| `anon-vec-0.1.1` | `vec::AnonVec::get_ref / vec::AnonVec::get_mut` | `std::ptr::const_ptr::add / std::ptr::mut_ptr::add` | [RUSTSEC-2025-0039](https://rustsec.org/advisories/RUSTSEC-2025-0039.html) |
| `fast_id_map-0.1.0` | `fast_map::FastMap::get` | `core::slice::get_unchecked_mut` | [RUSTSEC-2025-0034](https://rustsec.org/advisories/RUSTSEC-2025-0034.html) |
| `tanton_engine-1.0.0` | `search::Stack::offset` | `std::ptr::mut_ptr::offset` | [RUSTSEC-2025-0031](https://rustsec.org/advisories/RUSTSEC-2025-0031.html) |
| `bytes-1.11.0` | `bytes_mut::BytesMut::spare_capacity_mut` | `core::ptr::mut_ptr::add / core::slice::from_raw_parts_mut` | [RUSTSEC-2026-0007](https://rustsec.org/advisories/RUSTSEC-2026-0007.html) |
| `iced-x86-1.10.3` | `decoder::Decoder::new` | `core::slice::get_unchecked` | [RUSTSEC-2021-0068](https://rustsec.org/advisories/RUSTSEC-2021-0068.html) |
| `memmap2-0.9.10` | `os::MmapInner::flush / os::MmapInner::flush_async` | `std::ptr::mut_ptr::offset` | [RUSTSEC-2026-0186](https://rustsec.org/advisories/RUSTSEC-2026-0186.html) |
| `orx-pinned-vec-3.20.0` | `utils::slice::index_of_ptr` | `core::ptr::const_ptr::add` | [RUSTSEC-2025-0106](https://rustsec.org/advisories/RUSTSEC-2025-0106.html) |
| `scanner-0.1.0` | `Match::ptr / Match::get` | `std::ptr::const_ptr::offset` | [RUSTSEC-2025-0033](https://rustsec.org/advisories/RUSTSEC-2025-0033.html) |
| `redox_uefi_std-0.1.13` | `ffi::nstr` | `core::ptr::const_ptr::offset / core::ptr::__raw_ptr_deref__` | [RUSTSEC-2025-0032](https://rustsec.org/advisories/RUSTSEC-2025-0032.html) |
| `stack-0.3.0` | `array_vec::ArrayVec::insert_oob` | `core::intrinsics::copy / std::ptr::write` | [RUSTSEC-2020-0042](https://rustsec.org/advisories/RUSTSEC-2020-0042.html) |
| `auto_vec-0.2.0` | `AutoVec::iter / AutoVec::iter_mut` | `std::ptr::const_ptr::sub` | [RUSTSEC-2026-0133](https://rustsec.org/advisories/RUSTSEC-2026-0133.html) |
| `rtvm-interpreter-4.0.0` | `interpreter::Interpreter::program_counter` | `std::ptr::const_ptr::offset_from` | [RUSTSEC-2025-0131](https://rustsec.org/advisories/RUSTSEC-2025-0131.html) |
| `grcov-0.8.19` | `covdir::CDFileStats::get_coverage` | `core::slice::get_unchecked_mut` | [RUSTSEC-2025-0005](https://rustsec.org/advisories/RUSTSEC-2025-0005.html) |
| `slab-0.4.10` | `Slab::get_disjoint_mut` | `core::ptr::mut_ptr::add / core::mem::MaybeUninit::assume_init` | [RUSTSEC-2025-0047](https://rustsec.org/advisories/RUSTSEC-2025-0047.html) |
| `toodee-0.5.0` | `DrainCol::drop` | `core::intrinsics::copy` | [RUSTSEC-2025-0062](https://rustsec.org/advisories/RUSTSEC-2025-0062.html) |
| `xmas-elf-0.9.1` | `hash::HashTable::get_bucket` | `core::ptr::const_ptr::offset` | [RUSTSEC-2025-0018](https://rustsec.org/advisories/RUSTSEC-2025-0018.html) |
| `pprof-0.13.0` | `addr_validate::validate` | `std::slice::from_raw_parts` | [RUSTSEC-2024-0408](https://rustsec.org/advisories/RUSTSEC-2024-0408.html) |
| `partial_sort-0.1.2` | `partial_sort` | `core::slice::get_unchecked` | [RUSTSEC-2023-0016](https://rustsec.org/advisories/RUSTSEC-2023-0016.html) |
| `reorder-1.0.3` | `swap_index` | `std::slice::from_raw_parts_mut / std::vec::Vec::set_len` | [RUSTSEC-2021-0050](https://rustsec.org/advisories/RUSTSEC-2021-0050.html) |
| `toodee-0.5.0` | `toodee::TooDee::insert_row` | `core::ptr::mut_ptr::add / core::intrinsics::copy` | [RUSTSEC-2021-0028](https://rustsec.org/advisories/RUSTSEC-2021-0028.html) |
| `bam-0.1.2` | `bgzip::Block::load` | `std::vec::Vec::set_len` | [RUSTSEC-2021-0027](https://rustsec.org/advisories/RUSTSEC-2021-0027.html) |
| `smallvec-0.6.9` | `grow` | `core::intrinsics::copy_nonoverlapping / deallocate` | [RUSTSEC-2019-0012](https://rustsec.org/advisories/RUSTSEC-2019-0012.html) |
| `tremor-script-0.11.5` | `merge/patch` | `std::intrinsics::transmute` | [RUSTSEC-2021-0111](https://rustsec.org/advisories/RUSTSEC-2021-0111.html) |
| `ozone-0.1.0` | `Index/IndexMut` | `std::ptr::mut_ptr::offset / std::mem::uninitialized` | [RUSTSEC-2020-0022](https://rustsec.org/advisories/RUSTSEC-2020-0022.html) |
| `lexical-util-0.8.5` | `Bytes::read` | `core::mem::MaybeUninit::assume_init` | [RUSTSEC-2023-0086](https://rustsec.org/advisories/RUSTSEC-2023-0086.html) |
| `rdiff-0.1.2` | `window::Window::new` | `std::vec::Vec::set_len` | [RUSTSEC-2021-0094](https://rustsec.org/advisories/RUSTSEC-2021-0094.html) |

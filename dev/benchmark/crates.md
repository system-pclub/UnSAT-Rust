# 补充 crate 统计

采集时间：2026-11-22T01:50:53.938084+00:00。

| Link | crates.io | Date | GitHub Stars | Recent Downloads | Total Downloads | LoC | Unsafe STD Call Sites | # of bugs | Version or Commit Hash |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| https://github.com/SFBdragon/talc | https://crates.io/crates/talc | 2026-09-09 | 573 | 1197858 | 2694291 | 5255 | 52 | 6 | 5.0.3 |
| https://github.com/RedisLabsModules/redismodule-rs | https://crates.io/crates/redis-module | 2026-05-05 | 300 | 22530 | 830467 | 7509 | 28 | 4 | 2.0.8 |
| https://github.com/fschutt/azul | https://crates.io/crates/azul-css | 2026-11-19 | 6141 | 345247 | 643596 | 45570 | 147 | 1 | 0.0.8 |

Date 使用 crates.io 的 `updated_at`，表示该包最近更新时间；版本列固定为本次本地实验的版本，二者可能不同。GitHub Stars 来自对应仓库 API；Recent Downloads 为 crates.io `recent_downloads`（最近 90 天），Total Downloads 为该包所有版本累计下载量。原始 API 响应保存在 `metadata/`。

LoC 为原始归档 `src/**/*.rs` 中非空行数，包含注释，不含测试和示例目录。Unsafe STD Call Sites 是现有 MIRScan 报告中调用 `core/std/alloc` unsafe 函数的唯一 `(文件, 行, 列)` 数量；受当时编译特性和实例化覆盖范围限制，并非全特性静态词法统计。

`# of bugs` 为已有人工复核判为 TP 的源码行数，同一行多个 unsafe 调用合并；不是独立根因数，也不使用本次 Claude 的未复核输出。来源为根目录两个 `fast-findings*verdicts.tsv` 和 `dev/findings/verdicts.json`，具体位置见 `table-evidence.json`。

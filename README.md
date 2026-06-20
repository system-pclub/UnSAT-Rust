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







## How to verify a rule
```bash
$ ./x verify crates_inj/arenavec-0.1.1 --callsite src-common-rs-294-17 --rule rule-446 --report-json crates_inj/arenavec-0.1.1/report.json --test --compose-loop-bound 1
```


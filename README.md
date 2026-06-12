## Directory
- ./crates: A list of real world rust crates
- ./meta:   A list of metadata for each crate in ./crates
- ./rust:   std and core library from Rust compiler
- ./tools:  Helper tools for using this dataset

## Setup
```bash
# Initialize submodule 
$ git submodule update --init --recursive
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
$ ./x verify crates_inj/arenavec-0.1.1 --callsite src-common-rs-86-28 --rule rule-446 --report-json crates_inj/arenavec-0.1.1/report.json --test
```
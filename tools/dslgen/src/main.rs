use dslgen::{Catalog, Registry};
use std::{fs, path::PathBuf};

fn main() {
    if let Err(error) = run() {
        eprintln!("dslgen: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut inputs = vec![];
    let mut rust = PathBuf::from("tools/rust");
    let mut output = PathBuf::from("tools/dslgen/generated");
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--source" => inputs.push(PathBuf::from(args.next().ok_or("--source needs a path")?)),
            "--rust-src" => {
                rust = args
                    .next()
                    .ok_or("--rust-src needs the Rust checkout")?
                    .into()
            }
            "--out" => output = args.next().ok_or("--out needs a directory")?.into(),
            "--help" | "-h" => {
                println!("dslgen [--rust-src tools/rust] [--source PATH]... [--out DIRECTORY]\n\nDefaults: library/{{core,alloc,std}}/src.\nGenerates one API-independent type/member catalog.");
                return Ok(());
            }
            _ => return Err(format!("unknown argument {arg}; see --help")),
        }
    }
    if inputs.is_empty() {
        inputs.extend(
            ["library/core/src", "library/alloc/src", "library/std/src"].map(|p| rust.join(p)),
        );
    }
    let catalog = Catalog::generate(&Registry::load(&inputs)?)?;
    fs::create_dir_all(&output).map_err(|e| e.to_string())?;
    for (file, text) in [
        (
            "schema.json",
            serde_json::to_string_pretty(&catalog).map_err(|e| e.to_string())? + "\n",
        ),
        ("selectors.md", catalog.markdown()),
    ] {
        fs::write(output.join(file), text).map_err(|e| e.to_string())?;
    }
    eprintln!("{} files, {} type/payload definitions, {} ambiguous types, {} declaration diagnostics; generated {}", catalog.files.len(), catalog.definitions.len(), catalog.alternatives.len(), catalog.diagnostics.len(), output.display());
    Ok(())
}

use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use log::{LevelFilter, info};
use serde::Serialize;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use toml::Value as TomlValue;

#[derive(Debug, Clone, ValueEnum)]
enum LogLevel {
	Error,
	Warn,
	Info,
	Debug,
	Trace,
}

#[derive(Debug, Parser)]
#[command(name = "x")]
#[command(about = "Project CLI tooling")]
struct Cli {
	#[arg(long, global = true, value_enum, default_value_t = LogLevel::Info, env = "X_LOG_LEVEL")]
	log_level: LogLevel,

	#[command(subcommand)]
	command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
	/// Compile crates with MIR scan rustc and persist crate metadata + report.
	Sync,
	/// Placeholder command for future spec equality check.
	CheckSpecEqual,
}

#[derive(Debug, Serialize)]
struct MetaOutput {
	crate_dir: String,
	crate_name: Option<String>,
	description: Option<String>,
	cargo_toml: String,
	report: Value,
}

fn main() -> Result<()> {
	let cli = Cli::parse();
	init_logging(cli.log_level.to_level_filter())?;

	match cli.command {
		Commands::Sync => run_sync(&cli),
		Commands::CheckSpecEqual => run_check_spec_equal(&cli),
	}
}

impl LogLevel {
	fn to_level_filter(self) -> LevelFilter {
		match self {
			Self::Error => LevelFilter::Error,
			Self::Warn => LevelFilter::Warn,
			Self::Info => LevelFilter::Info,
			Self::Debug => LevelFilter::Debug,
			Self::Trace => LevelFilter::Trace,
		}
	}
}

fn init_logging(level: LevelFilter) -> Result<()> {
	env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(level.as_str()))
		.try_init()
		.context("failed to initialize logging")?;
	Ok(())
}

fn run_sync(cli: &Cli) -> Result<()> {
	let repo_root = find_repo_root()?;
	let crates_dir = repo_root.join("crates");
	let meta_dir = repo_root.join("meta");

	fs::create_dir_all(&meta_dir)
		.with_context(|| format!("failed to create meta directory: {}", meta_dir.display()))?;

	let mirscan_rustc = std::env::var("MIRSCAN_RUSTC").unwrap_or_else(|_| "mirscan".to_string());
	let crate_dirs = find_crates(&crates_dir)?;

	for crate_dir in crate_dirs {
		let crate_name = crate_dir
			.file_name()
			.and_then(|x| x.to_str())
			.unwrap_or("unknown");

		info!("syncing crate {crate_name} with rustc={mirscan_rustc}");

		compile_crate(&crate_dir, &mirscan_rustc)?;

		let report_path = crate_dir.join("report.json");
		let report_raw = fs::read_to_string(&report_path).with_context(|| {
			format!(
				"failed to read report.json for crate {} at {}",
				crate_name,
				report_path.display()
			)
		})?;
		let report: Value = serde_json::from_str(&report_raw).with_context(|| {
			format!(
				"report.json is not valid JSON for crate {} at {}",
				crate_name,
				report_path.display()
			)
		})?;

		let cargo_toml_path = crate_dir.join("Cargo.toml");
		let cargo_toml = fs::read_to_string(&cargo_toml_path).with_context(|| {
			format!(
				"failed to read Cargo.toml for crate {} at {}",
				crate_name,
				cargo_toml_path.display()
			)
		})?;

		let (parsed_name, parsed_description) = parse_package_name_and_description(&cargo_toml)
			.with_context(|| format!("failed to parse package metadata for crate {crate_name}"))?;

		let out = MetaOutput {
			crate_dir: crate_name.to_string(),
			crate_name: parsed_name,
			description: parsed_description,
			cargo_toml,
			report,
		};

		let out_path = meta_dir.join(format!("{}.json", crate_name));
		let out_json = serde_json::to_string_pretty(&out).context("failed to serialize meta output")?;
		fs::write(&out_path, out_json)
			.with_context(|| format!("failed to write meta file {}", out_path.display()))?;
	}

	Ok(())
}

fn run_check_spec_equal(_cli: &Cli) -> Result<()> {
	Ok(())
}

fn find_repo_root() -> Result<PathBuf> {
	let start = std::env::current_dir().context("failed to read current directory")?;
	for dir in start.ancestors() {
		if dir.join("crates").is_dir() {
			return Ok(dir.to_path_buf());
		}
	}
	anyhow::bail!(
		"could not locate repository root from {} (expected an ancestor containing crates/) ",
		start.display()
	);
}

fn find_crates(crates_dir: &Path) -> Result<Vec<PathBuf>> {
	let mut dirs = Vec::new();
	let entries = fs::read_dir(crates_dir)
		.with_context(|| format!("failed to read crates directory {}", crates_dir.display()))?;

	for entry in entries {
		let entry = entry.with_context(|| {
			format!("failed to read an entry in crates directory {}", crates_dir.display())
		})?;
		let path = entry.path();
		if path.is_dir() && path.join("Cargo.toml").is_file() {
			dirs.push(path);
		}
	}

	dirs.sort();
	Ok(dirs)
}

fn compile_crate(crate_dir: &Path, mirscan_rustc: &str) -> Result<()> {
	let status = Command::new("cargo")
		.arg("check")
		.current_dir(crate_dir)
		.env("RUSTC", mirscan_rustc)
		.stdout(Stdio::inherit())
		.stderr(Stdio::inherit())
		.status()
		.with_context(|| {
			format!(
				"failed to spawn cargo check in {} with RUSTC={}",
				crate_dir.display(),
				mirscan_rustc
			)
		})?;

	if !status.success() {
		anyhow::bail!(
			"cargo check failed in {} with RUSTC={} (exit: {:?})",
			crate_dir.display(),
			mirscan_rustc,
			status.code()
		);
	}

	Ok(())
}

fn parse_package_name_and_description(cargo_toml: &str) -> Result<(Option<String>, Option<String>)> {
	let manifest: TomlValue = cargo_toml
		.parse()
		.context("Cargo.toml content is not valid TOML")?;
	let package = manifest.get("package").and_then(TomlValue::as_table);
	let name = package
		.and_then(|table| table.get("name"))
		.and_then(TomlValue::as_str)
		.map(ToOwned::to_owned);
	let description = package
		.and_then(|table| table.get("description"))
		.and_then(TomlValue::as_str)
		.map(ToOwned::to_owned);

	Ok((name, description))
}

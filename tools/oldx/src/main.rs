use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use log::{LevelFilter, info};
use serde::Serialize;
use serde_json::Value;
use std::fs;
use std::ffi::OsString;
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
	report: Value,
}

fn main() -> Result<()> {
	let cli = Cli::parse();
	init_logging((&cli.log_level).to_level_filter())?;

	match cli.command {
		Commands::Sync => run_sync(),
		Commands::CheckSpecEqual => run_check_spec_equal(&cli),
	}
}

impl LogLevel {
	fn to_level_filter(&self) -> LevelFilter {
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

fn run_sync() -> Result<()> {
	let repo_root = find_repo_root()?;
	let crates_dir = repo_root.join("crates");
	let meta_dir = repo_root.join("meta");

	fs::create_dir_all(&meta_dir)
		.with_context(|| format!("failed to create meta directory: {}", meta_dir.display()))?;

	let mirscan_rustc = resolve_mirscan_rustc(&repo_root)?;
	let crate_dirs = find_crates(&crates_dir)?;

	for crate_dir in crate_dirs {
		let crate_name = crate_dir
			.file_name()
			.and_then(|x| x.to_str())
			.unwrap_or("unknown");
		let crate_dir_relative = crate_dir
			.strip_prefix(&repo_root)
			.unwrap_or(&crate_dir)
			.to_string_lossy()
			.into_owned();

		info!("syncing crate {crate_name} with rustc={mirscan_rustc}");

		let report_path = crate_dir.join("report.json");
		compile_crate(&crate_dir, &mirscan_rustc, &report_path)?;

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
			crate_dir: crate_dir_relative,
			crate_name: parsed_name,
			description: parsed_description,
			report,
		};

		let out_path = meta_dir.join(format!("{}.json", crate_name));
		let out_json = serde_json::to_string_pretty(&out).context("failed to serialize meta output")?;
		fs::write(&out_path, out_json)
			.with_context(|| format!("failed to write meta file {}", out_path.display()))?;
	}

	Ok(())
}

fn resolve_mirscan_rustc(repo_root: &Path) -> Result<String> {
	if let Ok(configured) = std::env::var("MIRSCAN_RUSTC") {
		return Ok(configured);
	}

	let local_raudit = repo_root.join("tools/mirscan/target/debug/raudit");
	if local_raudit.is_file() {
		return Ok(local_raudit.to_string_lossy().into_owned());
	}

	build_local_mirscan(repo_root)?;
	if local_raudit.is_file() {
		return Ok(local_raudit.to_string_lossy().into_owned());
	}

	if let Some(path) = find_executable_in_path("raudit") {
		return Ok(path.to_string_lossy().into_owned());
	}

	if let Some(path) = find_executable_in_path("mirscan") {
		return Ok(path.to_string_lossy().into_owned());
	}

	anyhow::bail!(
		"could not find mirscan rustc binary; set MIRSCAN_RUSTC or build tools/mirscan (expected tools/mirscan/target/debug/raudit)"
	)
}

fn build_local_mirscan(repo_root: &Path) -> Result<()> {
	let mirscan_manifest = repo_root.join("tools/mirscan/Cargo.toml");
	if !mirscan_manifest.is_file() {
		return Ok(());
	}

	let status = Command::new("cargo")
		.arg("build")
		.arg("--manifest-path")
		.arg(&mirscan_manifest)
		.current_dir(repo_root)
		.stdout(Stdio::inherit())
		.stderr(Stdio::inherit())
		.status()
		.with_context(|| {
			format!(
				"failed to build local mirscan with manifest {}",
				mirscan_manifest.display()
			)
		})?;

	if !status.success() {
		anyhow::bail!(
			"building local mirscan failed (manifest: {}, exit: {:?})",
			mirscan_manifest.display(),
			status.code()
		);
	}

	Ok(())
}

fn find_executable_in_path(name: &str) -> Option<PathBuf> {
	let path_var = std::env::var_os("PATH")?;
	for entry in std::env::split_paths(&path_var) {
		let candidate = entry.join(name);
		if candidate.is_file() {
			return Some(candidate);
		}
	}
	None
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

fn compile_crate(crate_dir: &Path, mirscan_rustc: &str, report_path: &Path) -> Result<()> {
	if report_path.exists() {
		fs::remove_file(report_path).with_context(|| {
			format!(
				"failed to remove existing report file {}",
				report_path.display()
			)
		})?;
	}

	let clean_status = Command::new("cargo")
		.arg("clean")
		.current_dir(crate_dir)
		.stdout(Stdio::inherit())
		.stderr(Stdio::inherit())
		.status()
		.with_context(|| format!("failed to spawn cargo clean in {}", crate_dir.display()))?;

	if !clean_status.success() {
		anyhow::bail!(
			"cargo clean failed in {} (exit: {:?})",
			crate_dir.display(),
			clean_status.code()
		);
	}

	let analysis_out: OsString = report_path.as_os_str().to_os_string();
	let status = Command::new("cargo")
		.arg("check")
		.current_dir(crate_dir)
		.env("RUSTC", mirscan_rustc)
		.env("ANALYSIS_OUT", analysis_out)
		.stdout(Stdio::inherit())
		.stderr(Stdio::inherit())
		.status()
		.with_context(|| {
			format!(
				"failed to spawn cargo check in {} with RUSTC={} and ANALYSIS_OUT={}",
				crate_dir.display(),
				mirscan_rustc,
				report_path.display(),
			)
		})?;

	if !status.success() {
		anyhow::bail!(
			"cargo check failed in {} with RUSTC={} and ANALYSIS_OUT={} (exit: {:?})",
			crate_dir.display(),
			mirscan_rustc,
			report_path.display(),
			status.code()
		);
	}

	Ok(())
}

fn parse_package_name_and_description(cargo_toml: &str) -> Result<(Option<String>, Option<String>)> {
	let manifest: TomlValue = toml::from_str(cargo_toml)
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

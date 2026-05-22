#![feature(rustc_private, stmt_expr_attributes)]
#![allow(
    clippy::manual_range_contains,
    clippy::useless_format,
    clippy::field_reassign_with_default,
    clippy::needless_lifetimes,
    rustc::diagnostic_outside_of_impl,
    rustc::untranslatable_diagnostic
)]

extern crate tracing;
extern crate rustc_abi;
extern crate rustc_data_structures;
extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_hir_analysis;
extern crate rustc_interface;
extern crate rustc_log;
extern crate rustc_metadata;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_hir_pretty;


use std::path::{Path, PathBuf};


use rustc_driver::Compilation;
use rustc_hir::def_id::LOCAL_CRATE;
use rustc_middle::ty::{ TyCtxt};
use rustc_session::config::{ErrorOutputType};
use rustc_session::{EarlyDiagCtxt};
pub mod report;
// pub mod invariants;
struct CompilerCalls {}



impl CompilerCalls {
    fn new() -> Self {
        CompilerCalls {}
    }
}

impl rustc_driver::Callbacks for CompilerCalls {
    fn after_analysis<'tcx>(
        &mut self,
        compiler: &rustc_interface::interface::Compiler,
        tcx: TyCtxt<'tcx>,
    ) -> Compilation {
        if tcx.sess.dcx().has_errors_or_delayed_bugs().is_some() {
            tcx.dcx().fatal("raudit cannot be run on programs that fail compilation");
        }

        let crate_fspath = match compiler.sess.io.output_dir {
            Some(ref p) => {
                let mut pp = p.clone();
                // pop target/debug/deps
                pp.pop();
                pp.pop();
                pp.pop();
                pp
            },
            None => return Compilation::Stop
        };
        let current_pwd = std::env::current_dir().unwrap_or_else(|e| {
            eprintln!("Failed to get current working directory: {}", e);
            std::process::exit(1);
        });

        if crate_fspath != current_pwd {
            
            return Compilation::Continue;
        }

        let report_output = std::env::var("ANALYSIS_OUT");
        if report_output.is_err() {
            // Not running in analysis mode - let compilation continue normally
            return Compilation::Continue;
        }
        let report_output = PathBuf::from(report_output.unwrap());
        
        eprintln!("Running analysis");  
        if let Some(parent) = report_output.parent() {
            match std::fs::create_dir_all(&parent) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Failed to create report output directory {:?}: {}", parent, e);
                    return Compilation::Stop;
                }
            }
        }
        let report = report::audit(tcx);
        let json_str = serde_json::to_string_pretty(&report).unwrap();
        match std::fs::write(&report_output, json_str.as_bytes()) {
            Ok(_) => {
                if report_output.is_file() {
                    eprintln!("[+] {:?}", report_output);
                } else {
                    eprintln!("Failed to persist report output file {:?}", report_output);
                }
            }
            Err(e) => {
                eprintln!("Failed to write report output file {:?}: {}", report_output, e);
            }
        }
        

        Compilation::Stop
    }
}


/// Execute a compiler with the given CLI arguments and callbacks.
fn run_compiler_and_exit(
    args: &[String],
    callbacks: &mut (dyn rustc_driver::Callbacks + Send),
) -> ! {
    // Invoke compiler, and handle return code.
    let exit_code =
        rustc_driver::catch_with_exit_code(move || rustc_driver::run_compiler(args, callbacks));
    std::process::exit(exit_code)
}



pub const DEFAULT_ARGS: &[&str] = &[
    "-Zalways-encode-mir",
    "-Zextra-const-ub-checks",
    "-Zmir-emit-retag",
    "-Zmir-opt-level=0",
    "-Zdeduplicate-diagnostics=no",
];


fn main() {
    let early_dcx = EarlyDiagCtxt::new(ErrorOutputType::default());

    // Snapshot a copy of the environment before `rustc` starts messing with it.
    // (`install_ice_hook` might change `RUST_BACKTRACE`.)
    // let env_snapshot = env::vars_os().collect::<Vec<_>>();

    let args = rustc_driver::catch_fatal_errors(|| rustc_driver::args::raw_args(&early_dcx))
        .unwrap_or_else(|_| std::process::exit(rustc_driver::EXIT_FAILURE));

    // Install the ctrlc handler that sets `rustc_const_eval::CTRL_C_RECEIVED`, even if
    // MIRI_BE_RUSTC is set.
    rustc_driver::install_ctrlc_handler();


    // Add an ICE bug report hook.
    // rustc_driver::install_ice_hook("https://github.com/rust-lang/miri/issues/new", |_| ());

    // Init loggers the Miri way.
    // init_early_loggers(&early_dcx);


    let mut rustc_args = vec![];

    // Note that we require values to be given with `=`, not with a space.
    // This matches how rustc parses `-Z`.
    // However, unlike rustc we do not accept a space after `-Z`.
    for arg in args {
        if rustc_args.is_empty() {
            // Very first arg: binary name.
            rustc_args.push(arg);
            // Also add the default arguments.
            rustc_args.extend(DEFAULT_ARGS.iter().map(ToString::to_string));
        } else {
            // Forward to rustc.
            rustc_args.push(arg);
        }
    }

    // eprintln!("rustc arguments: {:?}", rustc_args);

    run_compiler_and_exit(
        &rustc_args,
        &mut CompilerCalls::new(),
    )
}



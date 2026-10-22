//! Command-line interface for Lang4LLM

use clap::{Parser, Subcommand, ValueEnum};
use colored::*;
use lang4llm_ast::SourceMap;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::process;
use std::process::Command;

mod diagnostic;
mod json_output;

/// Output format selector.
#[derive(Clone, Copy, Debug, ValueEnum, Default)]
pub enum OutputFormat {
    /// Human-readable, coloured output (default)
    #[default]
    Human,
    /// Machine-readable JSON on stdout
    Json,
}

/// Lang4LLM compiler and type checker
#[derive(Parser)]
#[command(name = "lang4llm")]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Output format
    #[arg(long, global = true, value_enum, default_value = "human")]
    format: OutputFormat,

    /// Subcommand to run
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse and type-check a Lang4LLM file
    Check {
        /// Input file path
        file: PathBuf,

        /// Show AST before type checking
        #[arg(short, long)]
        debug: bool,

        /// Show verbose output
        #[arg(short, long)]
        verbose: bool,
    },
    /// Parse a Lang4LLM file (show AST)
    Parse {
        /// Input file path
        file: PathBuf,

        /// Use compact output format
        #[arg(short, long)]
        compact: bool,
    },
    /// Transpile a Lang4LLM file to Rust source
    Compile {
        /// Input file path
        file: PathBuf,

        /// Output file path (default: <input>.rs)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Check compiled Rust source with rustc
        #[arg(long)]
        check: bool,

        /// Compile and run the generated Rust source
        #[arg(long)]
        run: bool,

        /// Show verbose/debug output
        #[arg(short, long)]
        verbose: bool,
    },
    /// Format a Lang4LLM source file to canonical style
    ///
    /// Without flags: prints formatted output to stdout.
    /// With --write: overwrites the file in-place.
    /// With --check: exits 1 if the file is not already formatted.
    Fmt {
        /// Input files or directories
        #[arg(required = true)]
        files: Vec<PathBuf>,

        /// Check whether files are formatted (exit 1 if not)
        #[arg(long)]
        check: bool,

        /// Write formatted output back to the source file
        #[arg(long)]
        write: bool,
    },
    /// Evaluate a Lang4LLM implementation against benchmark expected outputs
    ///
    /// Single file: compile+run and optionally check against paired .json sidecar.
    /// Suite mode (--suite DIR): run all .lang4llm files in a directory and report aggregate results.
    Eval {
        /// Input .lang4llm file (omit when using --suite)
        file: Option<PathBuf>,

        /// Run all .lang4llm files in this directory
        #[arg(long, value_name = "DIR")]
        suite: Option<PathBuf>,

        /// Check output against expected strings in paired .json sidecar
        /// (auto-enabled if .json sidecar exists alongside the file)
        #[arg(long)]
        check: bool,

        /// Show full stdout from each run
        #[arg(short, long)]
        verbose: bool,
    },
}

fn main() {
    let cli = Cli::parse();
    let fmt = cli.format;

    match cli.command {
        Commands::Check { file, debug, verbose } => check_file(&file, debug, verbose, fmt),
        Commands::Parse { file, compact } => parse_file(&file, compact, fmt),
        Commands::Compile { file, output, check, run, verbose } => {
            compile_file(&file, output.as_deref(), check, run, verbose, fmt)
        }
        Commands::Fmt { files, check, write } => fmt_command(&files, check, write, fmt),
        Commands::Eval { file, suite, check, verbose } => {
            if let Some(dir) = suite {
                eval_suite(&dir, verbose);
            } else if let Some(f) = file {
                let passed = eval_file(&f, check, verbose);
                process::exit(if passed { 0 } else { 1 });
            } else {
                eprintln!("{} Provide a file or --suite <dir>", "error:".red().bold());
                process::exit(1);
            }
        }
    }
}

// ── JSON output helpers ───────────────────────────────────────────────────────

/// Emit a JSON DiagnosticOutput to stdout and exit with the given code.
fn emit_json_and_exit(output: &json_output::DiagnosticOutput, exit_code: i32) -> ! {
    println!("{}", serde_json::to_string_pretty(output).expect("JSON serialisation"));
    process::exit(exit_code);
}

/// Build a JSON success result (no errors).
fn json_success() -> json_output::DiagnosticOutput {
    json_output::DiagnosticOutput {
        success: true,
        errors: Vec::new(),
        result: Some(json_output::CommandResult { note: None }),
    }
}

/// Build a JSON failure result from type errors.
fn json_type_errors(
    errors: &[lang4llm_typeck::TypeError],
    file: &str,
) -> json_output::DiagnosticOutput {
    json_output::DiagnosticOutput {
        success: false,
        errors: errors
            .iter()
            .map(|e| json_output::type_error_to_diagnostic(e, file))
            .collect(),
        result: None,
    }
}

/// Build a JSON failure result from a parse error.
fn json_parse_error(e: &lang4llm_parser::ParseError, file: &str) -> json_output::DiagnosticOutput {
    json_output::DiagnosticOutput {
        success: false,
        errors: vec![json_output::parse_error_to_diagnostic(e, file)],
        result: None,
    }
}

fn check_file(path: &PathBuf, debug: bool, verbose: bool, fmt: OutputFormat) {
    let file_str = path.display().to_string();

    // T029: graceful handling for empty / unreadable files
    let source = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            match fmt {
                OutputFormat::Json => {
                    let out = json_output::DiagnosticOutput {
                        success: false,
                        errors: vec![json_output::DiagnosticError {
                            code: "E1001".to_string(),
                            severity: "error".to_string(),
                            message: format!("cannot read file: {e}"),
                            file: file_str,
                            span: None,
                            suggestion: None,
                            related: Vec::new(),
                        }],
                        result: None,
                    };
                    emit_json_and_exit(&out, 1);
                }
                OutputFormat::Human => {
                    eprintln!("{} Failed to read {}: {}", "error:".red().bold(), path.display(), e);
                    process::exit(1);
                }
            }
        }
    };

    if matches!(fmt, OutputFormat::Human) && verbose {
        println!("{} Parsing {}...", "→".cyan(), path.display());
    }

    // Parse
    let source_map = SourceMap::from_source(&source);
    let program = match lang4llm_parser::parse(&source, &source_map) {
        Ok(p) => p,
        Err(e) => match fmt {
            OutputFormat::Json => emit_json_and_exit(&json_parse_error(&e, &file_str), 1),
            OutputFormat::Human => {
                let diagnostic = diagnostic::report_parse_error(path, &source, &e);
                eprint!("{}", diagnostic);
                process::exit(1);
            }
        },
    };

    if matches!(fmt, OutputFormat::Human) && verbose {
        println!("{} Parsing complete", "✓".green());
    }

    // Show AST in debug mode (human only)
    if debug && matches!(fmt, OutputFormat::Human) {
        println!("\n{}", "─".repeat(60));
        println!("{}", "AST Debug Output".bold());
        println!("{}", "─".repeat(60));
        println!("{:#?}", program);
        println!("{}\n", "─".repeat(60));
    }

    if matches!(fmt, OutputFormat::Human) && verbose {
        println!("{} Type checking...", "→".cyan());
    }

    // Type check
    match lang4llm_typeck::check_program(&program) {
        Ok(()) => match fmt {
            OutputFormat::Json => emit_json_and_exit(&json_success(), 0),
            OutputFormat::Human => {
                if verbose {
                    println!("{} Type checking complete", "✓".green());
                }
                println!("{} {} type-checked successfully", "✓".green().bold(), path.display());
            }
        },
        Err(errors) => match fmt {
            OutputFormat::Json => {
                emit_json_and_exit(&json_type_errors(&errors, &file_str), 1);
            }
            OutputFormat::Human => {
                for err in &errors {
                    let diagnostic = diagnostic::report_type_error(path, &source, err);
                    eprint!("{}", diagnostic);
                }
                eprintln!(
                    "\n{} {} error{} found",
                    "✗".red().bold(),
                    errors.len(),
                    if errors.len() == 1 { "" } else { "s" }
                );
                process::exit(1);
            }
        },
    }
}

fn parse_file(path: &PathBuf, compact: bool, fmt: OutputFormat) {
    let file_str = path.display().to_string();
    let source = read_file(path);

    let source_map = SourceMap::from_source(&source);
    let program = match lang4llm_parser::parse(&source, &source_map) {
        Ok(p) => p,
        Err(e) => match fmt {
            OutputFormat::Json => emit_json_and_exit(&json_parse_error(&e, &file_str), 1),
            OutputFormat::Human => {
                let diagnostic = diagnostic::report_parse_error(path, &source, &e);
                eprint!("{}", diagnostic);
                process::exit(1);
            }
        },
    };

    match fmt {
        OutputFormat::Json => emit_json_and_exit(&json_success(), 0),
        OutputFormat::Human => {
            println!("{}", "─".repeat(60));
            println!("{} {}", "Parsed AST for".bold(), path.display());
            println!("{}", "─".repeat(60));
            if compact {
                println!("{:?}", program);
            } else {
                println!("{:#?}", program);
            }
            println!("\n{} Parsing completed successfully", "✓".green().bold());
        }
    }
}

fn compile_file(
    path: &PathBuf,
    output: Option<&std::path::Path>,
    check: bool,
    run: bool,
    verbose: bool,
    fmt: OutputFormat,
) {
    let file_str = path.display().to_string();
    let source = read_file(path);

    // --- Parse ---
    if matches!(fmt, OutputFormat::Human) && verbose {
        println!("{} Parsing {}...", "→".cyan(), path.display());
    }
    let source_map = SourceMap::from_source(&source);
    let program = match lang4llm_parser::parse(&source, &source_map) {
        Ok(p) => p,
        Err(e) => match fmt {
            OutputFormat::Json => emit_json_and_exit(&json_parse_error(&e, &file_str), 1),
            OutputFormat::Human => {
                let diagnostic = diagnostic::report_parse_error(path, &source, &e);
                eprint!("{}", diagnostic);
                process::exit(1); // exit code 1 = parse error
            }
        },
    };
    if matches!(fmt, OutputFormat::Human) && verbose {
        println!("{} Parsing complete", "✓".green());
    }

    // --- Type check ---
    if matches!(fmt, OutputFormat::Human) && verbose {
        println!("{} Type checking...", "→".cyan());
    }
    if let Err(errors) = lang4llm_typeck::check_program(&program) {
        match fmt {
            OutputFormat::Json => emit_json_and_exit(&json_type_errors(&errors, &file_str), 1),
            OutputFormat::Human => {
                for err in &errors {
                    let diagnostic = diagnostic::report_type_error(path, &source, err);
                    eprint!("{}", diagnostic);
                }
                eprintln!(
                    "\n{} {} error{} found",
                    "✗".red().bold(),
                    errors.len(),
                    if errors.len() == 1 { "" } else { "s" }
                );
                process::exit(1); // exit code 1 = type check error
            }
        }
    }
    if matches!(fmt, OutputFormat::Human) && verbose {
        println!("{} Type checking complete", "✓".green());
    }

    // --- Transpile ---
    if verbose {
        println!("{} Transpiling...", "→".cyan());
    }
    let rust_src = match lang4llm_transpiler::transpile(&program) {
        Ok(src) => src,
        Err(errors) => {
            eprintln!("{} Transpilation failed:", "✗".red().bold());
            for e in &errors {
                eprintln!("  {}", e);
            }
            process::exit(2); // exit code 2 = transpile error
        }
    };
    if verbose {
        println!("{} Transpilation complete", "✓".green());
    }

    // --- Determine output path ---
    let out_path: PathBuf = if let Some(p) = output {
        p.to_path_buf()
    } else {
        let mut p = path.clone();
        p.set_extension("rs");
        p
    };

    // --- Write Rust source ---
    fs::write(&out_path, &rust_src).unwrap_or_else(|e| {
        eprintln!(
            "{} Failed to write {}: {}",
            "error:".red().bold(),
            out_path.display(),
            e
        );
        process::exit(2);
    });

    if !check && !run {
        println!(
            "{} {} → {}",
            "✓".green().bold(),
            path.display(),
            out_path.display()
        );
        return;
    }

    // --- rustc check ---
    if check || run {
        if verbose {
            println!("{} Checking with rustc...", "→".cyan());
        }
        // Use a real temp file instead of /dev/null: rustc creates metadata files
        // alongside the output path, and writing to /dev causes permission errors
        // in sandboxed environments.
        let check_bin = std::env::temp_dir().join("lang4llm_check_bin");
        let status = Command::new("rustc")
            .args(["--edition", "2021", "--crate-type", "bin", "-o"])
            .arg(&check_bin)
            .arg(&out_path)
            .status();

        let _ = fs::remove_file(&check_bin); // clean up regardless of outcome
        match status {
            Ok(s) if s.success() => {
                if verbose {
                    println!("{} rustc check passed", "✓".green());
                }
            }
            Ok(_) => {
                eprintln!("{} rustc compilation failed", "✗".red().bold());
                process::exit(3); // exit code 3 = rustc error
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                eprintln!("{} rustc not found — skipping check", "⚠".yellow());
            }
            Err(e) => {
                eprintln!("{} Failed to run rustc: {}", "error:".red().bold(), e);
                process::exit(3);
            }
        }
    }

    // --- Run ---
    if run {
        if verbose {
            println!("{} Compiling and running...", "→".cyan());
        }

        // Compile to temp binary
        let tmp_bin = std::env::temp_dir().join("lang4llm_run_output");
        let compile_status = Command::new("rustc")
            .args(["--edition", "2021", "-o"])
            .arg(&tmp_bin)
            .arg(&out_path)
            .status();

        match compile_status {
            Ok(s) if s.success() => {}
            Ok(_) => {
                eprintln!("{} rustc compilation failed", "✗".red().bold());
                process::exit(3);
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                eprintln!("{} rustc not found — cannot run", "✗".red().bold());
                process::exit(3);
            }
            Err(e) => {
                eprintln!("{} Failed to run rustc: {}", "error:".red().bold(), e);
                process::exit(3);
            }
        }

        let run_status = Command::new(&tmp_bin).status();
        match run_status {
            Ok(s) => {
                if !s.success() {
                    process::exit(4); // exit code 4 = runtime error
                }
            }
            Err(e) => {
                eprintln!("{} Failed to execute output: {}", "error:".red().bold(), e);
                process::exit(4);
            }
        }

        // Cleanup
        let _ = fs::remove_file(&tmp_bin);
    }

    println!(
        "{} {} → {}",
        "✓".green().bold(),
        path.display(),
        out_path.display()
    );
}

// ── Eval harness ────────────────────────────────────────────────────────────

/// JSON sidecar format alongside each reference implementation.
#[derive(Deserialize)]
struct EvalSpec {
    #[allow(dead_code)]
    spec: String,
    #[allow(dead_code)]
    title: Option<String>,
    #[serde(default)]
    lang4llm_compatible: bool,
    #[serde(default)]
    expected: Vec<String>,
}

/// Transpile + compile + run a `.lang4llm` file, return captured stdout.
/// Returns `None` if any stage fails (parse / typecheck / transpile / rustc / run).
fn run_impl(path: &PathBuf, verbose: bool) -> Option<String> {
    let source = fs::read_to_string(path).ok()?;

    let source_map = SourceMap::from_source(&source);
    let program = lang4llm_parser::parse(&source, &source_map).ok()?;
    if lang4llm_typeck::check_program(&program).is_err() {
        if verbose {
            eprintln!("  {} type-check failed", "✗".red());
        }
        return None;
    }
    let rust_src = lang4llm_transpiler::transpile(&program).ok()?;

    let safe = path.to_string_lossy().replace(['.', '/', '\\'], "_");
    let tmp_src = std::env::temp_dir().join(format!("l4l_eval_{}.rs", safe));
    let tmp_bin = std::env::temp_dir().join(format!("l4l_eval_{}_bin", safe));

    fs::write(&tmp_src, &rust_src).ok()?;

    let compile_ok = Command::new("rustc")
        .args(["--edition", "2021", "-o"])
        .arg(&tmp_bin)
        .arg(&tmp_src)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    let _ = fs::remove_file(&tmp_src);

    if !compile_ok {
        if verbose {
            eprintln!("  {} rustc failed", "✗".red());
        }
        let _ = fs::remove_file(&tmp_bin);
        return None;
    }

    let out = Command::new(&tmp_bin).output().ok()?;
    let _ = fs::remove_file(&tmp_bin);

    Some(String::from_utf8_lossy(&out.stdout).to_string())
}

// ── fmt command ───────────────────────────────────────────────────────────────

/// JSON payload for `fmt --format json` results.
#[derive(serde::Serialize)]
struct FmtResult {
    file: String,
    /// `"formatted"` | `"already_formatted"` | `"check_failed"` | `"error"`
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

/// Collect all `.lang4llm` files from a list of paths (files or directories).
fn collect_lang4llm_files(paths: &[PathBuf]) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for p in paths {
        if p.is_dir() {
            if let Ok(rd) = std::fs::read_dir(p) {
                let mut dir_files: Vec<PathBuf> = rd
                    .flatten()
                    .map(|e| e.path())
                    .filter(|p| p.extension().map(|e| e == "lang4llm").unwrap_or(false))
                    .collect();
                dir_files.sort();
                files.extend(dir_files);
            }
        } else {
            files.push(p.clone());
        }
    }
    files
}

fn fmt_command(paths: &[PathBuf], check_only: bool, write_back: bool, output_fmt: OutputFormat) {
    let files = collect_lang4llm_files(paths);
    if files.is_empty() {
        eprintln!("{} no .lang4llm files found", "error:".red().bold());
        process::exit(1);
    }

    let mut any_unformatted = false;
    let mut json_results: Vec<FmtResult> = Vec::new();

    for path in &files {
        let source = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                match output_fmt {
                    OutputFormat::Human => {
                        eprintln!("{} {}: {e}", "error:".red().bold(), path.display());
                    }
                    OutputFormat::Json => {
                        json_results.push(FmtResult {
                            file: path.display().to_string(),
                            status: "error".to_string(),
                            error: Some(e.to_string()),
                        });
                    }
                }
                any_unformatted = true;
                continue;
            }
        };

        let formatted = match lang4llm_formatter::format_source(&source) {
            Ok(f) => f,
            Err(e) => {
                match output_fmt {
                    OutputFormat::Human => {
                        eprintln!("{} {}: {e}", "error:".red().bold(), path.display());
                    }
                    OutputFormat::Json => {
                        json_results.push(FmtResult {
                            file: path.display().to_string(),
                            status: "error".to_string(),
                            error: Some(e.to_string()),
                        });
                    }
                }
                any_unformatted = true;
                continue;
            }
        };

        let already_formatted = source == formatted;

        if check_only {
            if !already_formatted {
                any_unformatted = true;
                match output_fmt {
                    OutputFormat::Human => {
                        println!("{} {} is not formatted", "FAIL".red().bold(), path.display());
                    }
                    OutputFormat::Json => {
                        json_results.push(FmtResult {
                            file: path.display().to_string(),
                            status: "check_failed".to_string(),
                            error: None,
                        });
                    }
                }
            } else {
                match output_fmt {
                    OutputFormat::Human => {
                        println!("{} {}", "ok".green().bold(), path.display());
                    }
                    OutputFormat::Json => {
                        json_results.push(FmtResult {
                            file: path.display().to_string(),
                            status: "already_formatted".to_string(),
                            error: None,
                        });
                    }
                }
            }
        } else if write_back {
            if !already_formatted {
                if let Err(e) = fs::write(path, &formatted) {
                    eprintln!("{} {}: {e}", "error:".red().bold(), path.display());
                    any_unformatted = true;
                } else {
                    match output_fmt {
                        OutputFormat::Human => {
                            println!("{} formatted {}", "fmt".cyan().bold(), path.display());
                        }
                        OutputFormat::Json => {
                            json_results.push(FmtResult {
                                file: path.display().to_string(),
                                status: "formatted".to_string(),
                                error: None,
                            });
                        }
                    }
                }
            } else {
                match output_fmt {
                    OutputFormat::Human => {} // silent for already-formatted files in --write mode
                    OutputFormat::Json => {
                        json_results.push(FmtResult {
                            file: path.display().to_string(),
                            status: "already_formatted".to_string(),
                            error: None,
                        });
                    }
                }
            }
        } else {
            // Default: print to stdout (single file only makes sense; for many files just dump all)
            print!("{formatted}");
            if let OutputFormat::Json = output_fmt {
                json_results.push(FmtResult {
                    file: path.display().to_string(),
                    status: if already_formatted { "already_formatted" } else { "formatted" }
                        .to_string(),
                    error: None,
                });
            }
        }
    }

    if let OutputFormat::Json = output_fmt {
        println!(
            "{}",
            serde_json::to_string_pretty(&json_results).expect("JSON serialisation")
        );
    }

    if any_unformatted {
        process::exit(1);
    }
}

/// Evaluate a single `.lang4llm` file.
/// Returns `true` if the run succeeded and all expected strings were found.
fn eval_file(path: &PathBuf, check: bool, verbose: bool) -> bool {
    print!("{} {} ... ", "eval".cyan().bold(), path.display());

    // Load sidecar JSON if it exists (or if --check explicitly set)
    let json_path = path.with_extension("json");
    let spec: Option<EvalSpec> = if json_path.exists() || check {
        fs::read_to_string(&json_path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
    } else {
        None
    };

    // Check lang4llm_compatible flag
    if let Some(ref s) = spec {
        if !s.lang4llm_compatible {
            println!("{} (not applicable — {})",
                "SKIP".yellow().bold(),
                s.expected.first().map(|_| "incompatible").unwrap_or("no expected output"));
            return true; // not a failure
        }
    }

    // Run the implementation
    let output = match run_impl(path, verbose) {
        Some(o) => o,
        None => {
            println!("{}", "FAIL (run error)".red().bold());
            return false;
        }
    };

    if verbose {
        println!();
        println!("  stdout:\n{}", output.lines().map(|l| format!("    {}", l)).collect::<Vec<_>>().join("\n"));
    }

    // Check expected strings
    if let Some(spec) = spec {
        let mut all_pass = true;
        let mut missing = Vec::new();
        for expected in &spec.expected {
            if !output.contains(expected.as_str()) {
                all_pass = false;
                missing.push(expected.as_str());
            }
        }
        if all_pass {
            println!("{}", "PASS".green().bold());
            true
        } else {
            println!("{}", "FAIL".red().bold());
            for m in missing {
                eprintln!("  {} missing: {:?}", "✗".red(), m);
            }
            false
        }
    } else {
        // No sidecar — just confirm it ran
        println!("{}", "OK (ran, no checks)".green());
        true
    }
}

/// Run eval on all `.lang4llm` files in `dir`, report aggregate results.
fn eval_suite(dir: &PathBuf, verbose: bool) {
    let mut entries: Vec<PathBuf> = fs::read_dir(dir)
        .unwrap_or_else(|e| {
            eprintln!("{} Cannot read dir {}: {}", "error:".red().bold(), dir.display(), e);
            process::exit(1);
        })
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("lang4llm"))
        .collect();
    entries.sort();

    if entries.is_empty() {
        println!("{} No .lang4llm files found in {}", "⚠".yellow(), dir.display());
        return;
    }

    println!("{} Running {} benchmark(s) in {}\n",
        "eval".cyan().bold(), entries.len(), dir.display());

    let mut pass = 0usize;
    let mut fail = 0usize;
    let mut skip = 0usize;

    for path in &entries {
        // Peek at JSON to detect skip
        let json_path = path.with_extension("json");
        let is_skip = json_path.exists() && fs::read_to_string(&json_path)
            .ok()
            .and_then(|s| serde_json::from_str::<EvalSpec>(&s).ok())
            .map(|s| !s.lang4llm_compatible)
            .unwrap_or(false);

        if is_skip {
            skip += 1;
        } else if eval_file(path, true, verbose) {
            pass += 1;
        } else {
            fail += 1;
        }
    }

    let total = pass + fail;
    println!("\n{}", "─".repeat(50));
    println!("Results: {}/{} passed, {} skipped (not applicable)",
        pass, total, skip);
    if fail == 0 {
        println!("{}", "All benchmarks passed!".green().bold());
    } else {
        println!("{} {} benchmark(s) failed", "✗".red().bold(), fail);
        process::exit(1);
    }
}

// ── File I/O helper ──────────────────────────────────────────────────────────

fn read_file(path: &PathBuf) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("{} Failed to read {}: {}", "error:".red().bold(), path.display(), e);
        process::exit(1);
    })
}

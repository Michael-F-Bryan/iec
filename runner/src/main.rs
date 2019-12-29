//! The `main` function for the `iec` compiler.
//!
//! This crate doesn't really add new functionality to the compiler, instead it
//! glues together the front-end ([`iec_syntax`]), middle-end ([`iec`]), and
//! back-end to produce a functional compilation tool.

use codespan::{ByteOffset, ByteSpan, CodeMap, FileMap};
use codespan_reporting::termcolor::{ColorChoice, StandardStream};
use codespan_reporting::{Diagnostic, Label};
use failure::{Error, ResultExt};
use iec::{CompilationUnit, Diagnostics};
use iec_syntax::File;
use slog::{Drain, Level, Logger};
use slog_derive::KV;
use specs::World;
use std::str::FromStr;
use std::time::Instant;
use structopt::StructOpt;

fn main() {
    let args = Args::from_args();
    let logger = create_logger(args.verbosity);

    if let Err(e) = run(&args, &logger) {
        slog::error!(logger, "{}", e);
        for cause in e.iter_causes() {
            slog::warn!(logger, "Caused by: {}", cause);
        }

        drop(logger);

        let bt = e.backtrace().to_string();

        if !bt.trim().is_empty() {
            eprintln!("{}", bt);
        }

        std::process::exit(1);
    }
}

fn run(args: &Args, logger: &Logger) -> Result<(), Error> {
    slog::info!(logger, "Started the application"; &args);
    let start = Instant::now();
    let mut map = CodeMap::new();

    let fm = map
        .add_filemap_from_disk(&args.file)
        .context("Unable to read the file into memory")?;

    slog::debug!(logger, "Read the file to disk"; 
        "filename" => fm.name().as_ref().display(),
        "size" => fm.src().len());

    let syntax_logger = logger.new(slog::o!("stage" => "syntactic-analysis"));
    slog::debug!(syntax_logger, "Starting syntactic analysis");
    let start_syntax = Instant::now();

    let file = match syntactic_analysis(&fm) {
        Ok(f) => f,
        Err(e) => {
            let ss = StandardStream::stdout(ColorChoice::Auto);
            codespan_reporting::emit(ss, &map, &e)?;
            return Ok(());
        }
    };

    let duration = Instant::now() - start_syntax;
    slog::debug!(syntax_logger, "Finished syntactic analysis"; 
        "execution-time" => format_args!("{}.{:03}s", duration.as_secs(), duration.subsec_millis()));

    let mut diags = Diagnostics::new();

    let semantic_logger = logger.new(slog::o!("stage" => "semantic-analysis"));
    slog::debug!(semantic_logger, "Started semantic analysis");
    let start_semantics = Instant::now();

    let (_world, cu) = semantic_analysis(file, &mut diags, logger);

    if diags.has_errors() {
        let mut ss = StandardStream::stdout(ColorChoice::Auto);
        for diagnostic in diags.diagnostics() {
            codespan_reporting::emit(&mut ss, &map, diagnostic)?;
        }
        return Ok(());
    }

    let duration = Instant::now() - start_semantics;
    slog::debug!(semantic_logger, "Finished semantic analysis";
        "execution-time" => format_args!("{}.{:03}s", duration.as_secs(), duration.subsec_millis()));

    let duration = Instant::now() - start;
    slog::info!(logger, "Compilation finished"; 
        "execution-time" => format_args!("{}.{:03}s", duration.as_secs(), duration.subsec_millis()));

    slog::debug!(logger, "{:#?}", cu);
    Ok(())
}

fn semantic_analysis(
    file: File,
    diags: &mut Diagnostics,
    logger: &Logger,
) -> (World, CompilationUnit) {
    let logger = logger.new(slog::o!("stage" => "semantic-analysis"));
    iec::process(file, diags, &logger)
}

fn syntactic_analysis(file: &FileMap) -> Result<File, Diagnostic> {
    let offset = ByteOffset(file.span().start().0 as i64 - 2);

    iec_syntax::File::from_str(file.src())
        .map_err(|e| e.map_location(|l| l - offset))
        .map_err(|e| match e {
            lalrpop_util::ParseError::InvalidToken { location } => {
                Diagnostic::new_error("Invalid token").with_label(Label::new_primary(
                    ByteSpan::from_offset(location, ByteOffset(1)),
                ))
            }

            lalrpop_util::ParseError::ExtraToken {
                token: (start, tok, end),
            } => Diagnostic::new_error(format!("Encountered \"{}\" when it wasn't expected", tok))
                .with_label(Label::new_primary(ByteSpan::new(start, end))),

            lalrpop_util::ParseError::UnrecognizedToken {
                token: Some((start, tok, end)),
                expected,
            } => Diagnostic::new_error(format!(
                "Found \"{}\" but was expecting {}",
                tok,
                expected.join(", ")
            ))
            .with_label(Label::new_primary(ByteSpan::new(start, end))),

            lalrpop_util::ParseError::UnrecognizedToken {
                token: None,
                expected,
            } => Diagnostic::new_error(format!("Expected one of {}", expected.join(", "))),

            lalrpop_util::ParseError::User { error } => Diagnostic::new_error(error.to_string()),
        })
}

#[derive(Debug, Clone, PartialEq, StructOpt, KV)]
pub struct Args {
    #[structopt(help = "The file to compile")]
    pub file: String,
    #[structopt(
        short = "v",
        long = "verbose",
        parse(from_occurrences),
        help = "Generate more verbose output"
    )]
    pub verbosity: u32,
}

fn create_logger(verbosity: u32) -> Logger {
    let decorator = slog_term::TermDecorator::new().stderr().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let level = match verbosity {
        0 => Level::Warning,
        1 => Level::Info,
        2 => Level::Debug,
        _ => Level::Trace,
    };

    slog::Logger::root(drain.filter_level(level).fuse(), slog::o!())
}

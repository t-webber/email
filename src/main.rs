//! CLI to send emails from terminal

#![warn(
    missing_docs,
    warnings,
    deprecated_safe,
    future_incompatible,
    keyword_idents,
    let_underscore,
    nonstandard_style,
    refining_impl_trait,
    rust_2018_compatibility,
    rust_2018_idioms,
    rust_2021_compatibility,
    rust_2024_compatibility,
    unused,
    clippy::all,
    clippy::pedantic,
    clippy::style,
    clippy::perf,
    clippy::complexity,
    clippy::correctness,
    clippy::restriction,
    clippy::nursery,
    clippy::cargo
)]
#![expect(clippy::multiple_crate_versions, reason = "used crates need this")]
#![expect(clippy::blanket_clippy_restriction_lints, reason = "warn all lints")]
#![expect(
    clippy::implicit_return,
    clippy::question_mark_used,
    reason = "bad lint"
)]
#![expect(clippy::single_call_fn, reason = "style")]

mod email;
use clap::Parser;
use email::send;

/// Arguments required to send an email
#[derive(Parser)]
struct MailArguments {
    /// Email's body
    #[arg(short, long)]
    body: Option<String>,
    /// Email of the sender
    #[arg(short, long)]
    from: String,
    /// Name of the sender
    #[arg(short, long)]
    name: Option<String>,
    /// Password
    #[arg(short, long)]
    password: String,
    /// Email's subject
    #[arg(short, long)]
    subject: Option<String>,
    /// Lists of recipients
    #[arg(short, long, required = true, num_args = 1..)]
    to: Vec<String>,
    /// Enable logging
    #[arg(short, long)]
    verbose: bool,
}

/// Conditional logging for debugging
#[expect(clippy::print_stderr, reason = "logging for debugging")]
fn log_eprint(msg: &str, verbose: bool) {
    if verbose {
        eprint!("{msg}...{}\r", " ".repeat(10));
    }
}

fn main() -> Result<(), String> {
    let args = MailArguments::parse();
    let verbose = args.verbose;
    send(args).map_err(|err| if verbose { err } else { String::new() })
}

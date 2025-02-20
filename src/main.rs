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
use email::send;

/// Arguments required to send an email
struct MailArguments {
    /// Email's body
    body: Option<String>,
    /// Email of the sender
    from: String,
    /// Name of the sender
    name: Option<String>,
    /// Password
    password: String,
    /// Email's subject
    subject: Option<String>,
    /// Lists of recipients
    to: Vec<String>,
    /// Enable logging
    verbose: bool,
}

/// Conditional logging for debugging
#[expect(clippy::print_stderr, reason = "logging for debugging")]
fn log_eprint(msg: &str, verbose: bool) {
    if verbose {
        eprintln!("{msg}");
    }
}

fn main() -> Result<(), ()> {
    let args = MailArguments {
        name: None,
        from: "example@example.com".into(),
        to: vec!["example@example.com".into()],
        password: "some_password".into(),
        subject: None,
        body: None,
        verbose: true,
    };
    let verbose = args.verbose;
    send(args).map_err(|err| {
        log_eprint(&format!("\n{err}"), verbose);
    })
}

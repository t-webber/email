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
#![expect(clippy::pattern_type_mismatch, reason = "convenience")]
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
use core::fmt::Write as _;
use email::send;
use std::{
    fs::{self, create_dir_all, read_to_string},
    path::PathBuf,
};

/// Defines Clap Parser structs with different rules
macro_rules! make_parser_args {
    ($args:ident, $type:ty) => {
        /// Clap Parser struct
        #[derive(Parser)]
        struct $args {
            /// Email's body
            #[arg(short, long)]
            body: Option<String>,
            /// Email of the sender
            #[arg(short, long)]
            from: $type,
            /// Name of the sender
            #[arg(short, long)]
            name: Option<String>,
            /// Password
            #[arg(short, long)]
            password: $type,
            /// Enable storing of credentials
            ///
            /// When stored, the credentials won't be asked again.
            #[arg(long)]
            store: bool,
            /// Email's subject
            #[arg(short, long)]
            subject: Option<String>,
            /// Lists of recipients
            #[arg(short, long, required = true, num_args = 1..)]
            to: Vec<String>,
            /// Enable logging
            #[arg(long)]
            verbose: bool,
        }
    };
}

make_parser_args!(MailArguments, String);
make_parser_args!(OptionalMailArguments, Option<String>);

impl MailArguments {
    /// Parses the CLI input depending on whether credentials were stored in the past
    fn new(path_res: &Result<PathBuf, String>) -> Self {
        if let Ok(path) = path_res {
            if let Ok(content) = read_to_string(path) {
                let mut lines = content.lines();
                if let Some(stored_from) = lines.next() {
                    if let Some(stored_password) = lines.next() {
                        return Self::new_with_credentials(stored_from, stored_password);
                    }
                }
            }
        }
        Self::parse()
    }

    /// Parses the CLI input without needing origin email and password
    fn new_with_credentials(stored_from: &str, stored_password: &str) -> Self {
        let args = OptionalMailArguments::parse();
        Self {
            body: args.body,
            from: args.from.unwrap_or_else(|| stored_from.to_owned()),
            name: args.name,
            password: args.password.unwrap_or_else(|| stored_password.to_owned()),
            subject: args.subject,
            to: args.to,
            verbose: args.verbose,
            store: args.store,
        }
    }
}

/// Returns the path where the credentials are stored
fn get_credentials_path() -> Result<PathBuf, String> {
    let cache_path = dirs::cache_dir()
        .ok_or_else(|| "Failed to get cache directory.".to_owned())?
        .join("email-rs");
    create_dir_all(&cache_path)
        .map_err(|err| format!("Failed to create cache directory: {err:?}."))?;
    Ok(cache_path.join("data"))
}

/// Sends the email with the given arguments, and stores the credentials if wanted by the user.
fn email(path: Result<PathBuf, String>, args: &MailArguments) -> Result<(), String> {
    send(args)?;
    if args.store {
        fs::write(path?, format!("{}\n{}", args.from, args.password))
            .map_err(|err| format!("Failed to store credentials: {err:?}"))
    } else {
        Ok(())
    }
}

//"/home/bob/.cache/email-rs/data"

fn main() -> Result<(), ()> {
    let mut result = String::new();
    let path = get_credentials_path();
    let args = MailArguments::new(&path);
    if args.verbose {
        if let Err(err) = &path {
            writeln!(result, "Failed to access stored credentials: {err}").unwrap();
        }
    }
    let res = email(path, &args);
    if let Err(err) = res {
        if args.verbose {
            writeln!(result, "An error occurred, email not sent. Check your credentials, the destination email and that you are connected to the internet.\n\nError details: {err}").unwrap();
        } else {
            writeln!(
                result,
                "Email not sent. Use --verbose flag for more information."
            )
            .unwrap();
        }
    }
    #[expect(
        clippy::print_stderr,
        reason = "inform user of failure in a pretty way"
    )]
    if result.is_empty() {
        #[expect(clippy::print_stdout, reason = "clean verbose logs")]
        if args.verbose {
            println!("Email sent          ");
        }
        Ok(())
    } else {
        eprintln!("{result}");
        Err(())
    }
}

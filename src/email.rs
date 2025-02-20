//! Back-end to send the email corresponding to a [`MailArgument`]

use lettre::message::Mailbox;
use lettre::message::MultiPart;
use lettre::message::SinglePart;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::response::Response;
use lettre::Message;
use lettre::SmtpTransport;
use lettre::Transport as _;

use crate::MailArguments;

/// Checks if an email is valid
fn is_valid_email(email: &str) -> Result<(), String> {
    if email.contains('@')
        && email.contains('.')
        && email.len() > 4
        && email.chars().filter(|ch| *ch == '@').count() == 1
        && email.chars().last().unwrap_or('.').is_ascii_alphabetic()
    {
        Ok(())
    } else {
        Err(format!("Email {email} is invalid"))
    }
}

/// Builds the email
fn build_email(
    from_name: Option<String>,
    from_email: &str,
    to_emails: &[String],
    subject: Option<String>,
    body: Option<String>,
) -> Result<Message, String> {
    let mut email_msg = Message::builder();

    let from = from_name.map_or_else(
        || from_email.to_owned(),
        |name| format!("{name} <{from_email}>"),
    );

    email_msg = email_msg.from(
        from.parse::<Mailbox>()
            .map_err(|err| format!("Invalid from: {err}"))?,
    );

    for email in to_emails {
        email_msg = email_msg.to(email
            .parse::<Mailbox>()
            .map_err(|err| format!("Invalid to: {err}"))?);
    }

    let body_part = SinglePart::html(body.unwrap_or_default());

    email_msg
        .subject(subject.unwrap_or_default())
        .multipart(MultiPart::mixed().singlepart(body_part))
        .map_err(|err| format!("Could not build email: {err}"))
}

/// Builds the [`SmtpTransport`]
fn build_mailer(from: String, password: String) -> Result<SmtpTransport, String> {
    let smtp_creds = Credentials::new(from, password);

    Ok(lettre::SmtpTransport::relay("smtp.gmail.com")
        .map_err(|err| format!("Could not connect to gmail: {err}"))?
        .credentials(smtp_creds)
        .build())
}

/// Main function to send the function
pub fn send(mail_params: MailArguments) -> Result<Response, String> {
    eprint!("Checking emails...     \r");

    is_valid_email(&mail_params.from)?;

    for email in &mail_params.to {
        is_valid_email(email)?;
    }

    eprint!("Building email...     \r");

    let email = build_email(
        mail_params.name,
        &mail_params.from,
        &mail_params.to,
        mail_params.subject,
        mail_params.body,
    )?;
    let mailer = build_mailer(mail_params.from, mail_params.password)?;

    eprint!("Sending email...      \r");

    mailer
        .send(&email)
        .map_err(|err| format!("Could not send email: {err:?}"))
}

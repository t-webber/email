//! Back-end to send the email corresponding to the given email parameters

use lettre::message::Mailbox;
use lettre::message::MultiPart;
use lettre::message::SinglePart;
use lettre::transport::smtp::authentication::Credentials;
use lettre::Message;
use lettre::SmtpTransport;
use lettre::Transport as _;

use crate::MailArguments;

/// Builds the email
fn build_email(
    from_name: Option<&String>,
    from_email: &str,
    to_emails: &[String],
    subject: Option<&String>,
    body: Option<&String>,
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

    let body_part = SinglePart::plain(body.cloned().unwrap_or_default());

    email_msg
        .subject(subject.cloned().unwrap_or_default())
        .multipart(MultiPart::mixed().singlepart(body_part))
        .map_err(|err| format!("Could not build email: {err}"))
}

/// Builds the [`SmtpTransport`]
fn build_mailer(from: &str, password: &str) -> Result<SmtpTransport, String> {
    let smtp_credentials = Credentials::new(from.to_owned(), password.to_owned());

    Ok(lettre::SmtpTransport::relay("smtp.gmail.com")
        .map_err(|err| format!("Could not connect to gmail: {err}"))?
        .credentials(smtp_credentials)
        .build())
}

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

/// Main function to send the function
pub fn send(mail_params: &MailArguments) -> Result<(), String> {
    is_valid_email(&mail_params.from)?;

    for email in &mail_params.to {
        is_valid_email(email)?;
    }

    let email = build_email(
        mail_params.name.as_ref(),
        &mail_params.from,
        &mail_params.to,
        mail_params.subject.as_ref(),
        mail_params.body.as_ref(),
    )?;
    let mailer = build_mailer(&mail_params.from, &mail_params.password)?;

    let response = mailer
        .send(&email)
        .map_err(|err| format!("Could not send email: {err:?}"))?;

    if response.is_positive() {
        Ok(())
    } else {
        Err(format!(
            "Unknown error. The email may have been sent. Server returned code {}.",
            response.code()
        ))
    }
}

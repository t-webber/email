use lettre::message;
use lettre::transport::smtp::authentication;
use lettre::transport::smtp::response;
use lettre::Transport;

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

fn build_mailer(from: String, password: String) -> Result<lettre::SmtpTransport, String> {
    let smtp_creds = authentication::Credentials::new(from, password);

    Ok(lettre::SmtpTransport::relay("smtp.gmail.com")
        .map_err(|err| format!("Could not connect to gmail: {err}"))?
        .credentials(smtp_creds)
        .build())
}

fn build_email(
    from_name: Option<String>,
    from_email: &str,
    to_emails: &[String],
    subject: Option<String>,
    body: Option<String>,
) -> Result<lettre::Message, String> {
    let mut email_msg = lettre::Message::builder();

    let from = match from_name {
        Some(name) => format!("{name} <{from_email}>"),
        None => from_email.to_owned(),
    };

    email_msg = email_msg.from(
        from.parse::<message::Mailbox>()
            .map_err(|err| format!("Invalid from: {err}"))?,
    );

    for email in to_emails {
        email_msg = email_msg.to(email
            .parse::<message::Mailbox>()
            .map_err(|err| format!("Invalid to: {err}"))?);
    }

    let body_part = message::SinglePart::html(body.unwrap_or_default());

    email_msg
        .subject(subject.unwrap_or_default())
        .multipart(message::MultiPart::mixed().singlepart(body_part))
        .map_err(|err| format!("Could not build email: {err}"))
}

fn send(mail_params: MailArguments) -> Result<response::Response, String> {
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

    let result: Result<response::Response, String> = mailer
        .send(&email)
        .map_err(|err| format!("Could not send email: {err:?}"));

    result
}

struct MailArguments {
    name: Option<String>,
    from: String,
    to: Vec<String>,
    password: String,
    subject: Option<String>,
    body: Option<String>,
}

fn main() {
    let args = MailArguments {
        name: None,
        from: "example@example.com".into(),
        to: vec!["example@example.com".into()],
        password: "somepassword".into(),
        subject: None,
        body: None,
    };
    match send(args) {
        Ok(rep) if rep.is_positive() => {
            eprintln!("Email sent. Server returned code {}.", rep.code(),)
        }
        Ok(rep) => {
            eprintln!(
                "Unknown error. The email may have been sent. Server returned code {}.",
                rep.code(),
            )
        }
        Err(msg) => eprintln!("\n{msg}"),
    }
}

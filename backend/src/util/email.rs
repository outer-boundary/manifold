use color_eyre::{eyre::eyre, Result};
use lettre::{
    message::{header::ContentType, Mailbox, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Tokio1Executor,
};

use super::configuration::get_config;

#[tracing::instrument(skip(recipient_first_name, recipient_last_name, subject, html_content, text_content),fields(subject = %subject.clone().into()))]
pub async fn send_email(
    sender_email: Option<String>,
    recipient_email: String,
    recipient_first_name: String,
    recipient_last_name: String,
    subject: impl Into<String> + std::clone::Clone,
    html_content: impl Into<String>,
    text_content: impl Into<String>,
) -> Result<()> {
    let config = get_config()?;

    let email = lettre::Message::builder()
        .from(Mailbox::new(
            Some("Manifold".into()),
            sender_email
                .unwrap_or(config.email.host_user.clone())
                .parse()?,
        ))
        .to(Mailbox::new(
            Some([recipient_first_name, recipient_last_name].join(" ")),
            recipient_email.parse()?,
        ))
        .subject(subject.clone())
        .multipart(
            MultiPart::alternative()
                .singlepart(
                    SinglePart::builder()
                        .header(ContentType::TEXT_PLAIN)
                        .body(text_content.into()),
                )
                .singlepart(
                    SinglePart::builder()
                        .header(ContentType::TEXT_HTML)
                        .body(html_content.into()),
                ),
        )?;

    let credentials = Credentials::new(config.email.host_user, config.email.host_user_password);

    let mailer: AsyncSmtpTransport<Tokio1Executor> =
        AsyncSmtpTransport::<Tokio1Executor>::relay(&config.email.host)?
            .credentials(credentials)
            .build();

    match mailer.send(email).await {
        Ok(_) => {
            tracing::debug!(
                "Successfully sent email with subject '{}' to recipient '{}'",
                subject.clone().into(),
                recipient_email
            );
            Ok(())
        }
        Err(err) => {
            tracing::error!(
                "Unable to send email with subject '{}' to recipient '{}': {}",
                subject.clone().into(),
                recipient_email,
                err
            );
            Err(eyre!(
                "Unable to send email with subject '{}' to recipient '{}': {}",
                subject.clone().into(),
                recipient_email,
                err
            ))
        }
    }
}

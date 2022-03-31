use std::env;

use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub async fn send_mail(file_path: String) {
    let smtp_user;
    match env::var("SMTP_USER") {
        Ok(val) => smtp_user = val,
        Err(_e) => smtp_user = "none".to_string(),
    }

    let smtp_password;
    match env::var("SMTP_PASSWORD") {
        Ok(val) => smtp_password = val,
        Err(_e) => smtp_password = "none".to_string(),
    }

    let smtp_server;
    match env::var("SMTP_SERVER") {
        Ok(val) => smtp_server = val,
        Err(_e) => smtp_server = "none".to_string(),
    }

    if smtp_password == "none" {
        warn!("SMTP_PASSWORD not defined. Cant send mail.");
        return;
    }

    let email = Message::builder()
        .from("Nitro <noreply@intrusion.detection>".parse().unwrap())
        .reply_to("noreply@intrusion.detection".parse().unwrap())
        .to("marius.kriegerowski@gmail.com".parse().unwrap())
        .subject("Intrusion Detected")
        .body(String::from(format!("File: {}", file_path)))
        .unwrap();

    let creds = Credentials::new(smtp_user, smtp_password);

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay(&*smtp_server)
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => debug!("Email sent successfully"),
        Err(e) => warn!("Could not send email: {:?}", e),
    }
}
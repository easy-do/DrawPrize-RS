use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use log::info;

use common::config::SystemConf;
use common::error::MyError;
use model::email::SendEMailModel;

pub fn send_email(send_mail_model: SendEMailModel, system_conf: &SystemConf) -> Result<bool, MyError> {
    let email = Message::builder()
        .from(system_conf.email.smtp_username.parse().unwrap())
        .to(send_mail_model.to.parse().unwrap())
        .subject(send_mail_model.sub)
        .body(send_mail_model.body).unwrap();
    let creds = Credentials::new(system_conf.email.smtp_username.clone(), system_conf.email.smtp_password.clone());
    let mailer_builder = SmtpTransport::relay(&system_conf.email.smtp_server).or_else(|e1| {
        Err(MyError::ServerError(e1.to_string()))
    })?;

    let mailer = mailer_builder.credentials(creds)
        .build();
    match mailer.send(&email) {
        Ok(res) => {
            info!("send mail res:{:?}",res);
            Ok(true)
        }
        Err(e) => {
            Err(MyError::ServerError(format!("Could not send the email: {:?}", e)))
        }
    }
}
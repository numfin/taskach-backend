use juniper::ID;
use serde_json::{json, Value};

use crate::app_env::get_env;

pub struct MailTemplate {
    pub id: String,
    pub data: Value,
}
impl MailTemplate {
    pub fn register_confirmation(activation_id: &ID, email: &String) -> MailTemplate {
        MailTemplate {
            id: get_env::sendgrid_template_register(),
            data: json!({
                "confirm_registration_link":
                    format!(
                        "https://app.taskach.ru/confirm?id={}&email={}",
                        activation_id, email
                    )
            }),
        }
    }
}

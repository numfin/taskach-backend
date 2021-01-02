pub mod templates;
use reqwest;
use serde_json::{json, Value};
use templates::MailTemplate;

use crate::app_env::get_env;

fn form_data(to: &String, template: MailTemplate) -> Value {
    json!({
      "personalizations": [
        {
          "to": [{ "email": to }],
          "dynamic_template_data": template.data
        }
      ],
      "from": {
        "email": get_env::sendgrid_from_email(),
        "name": get_env::sendgrid_from_name()
      },
      "template_id": template.id
    })
}

pub async fn send_mail(to: &String, template: MailTemplate) -> Result<(), String> {
    let client = reqwest::Client::new();
    client
        .post("https://api.sendgrid.com/v3/mail/send")
        .bearer_auth(get_env::sendgrid_api_key())
        .json(&form_data(to, template))
        .send()
        .await
        .map_err(|err| err.to_string())?;
    Ok(())
}

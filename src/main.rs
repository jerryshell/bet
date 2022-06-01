use lettre::{smtp::authentication::Credentials, SmtpClient, Transport};
use lettre_email::{mime, EmailBuilder};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::BufReader,
    path::Path,
};

#[derive(Debug, Serialize, Deserialize)]
struct EmailConfig {
    smtp_domain: String,
    from_email_address: String,
    from_email_password: String,
    subject: String,
    html_body: String,
    attachment_file_name: String,
}

#[derive(Debug)]
struct Entity {
    to_email_address: String,
    file_path: String,
}

fn get_entity_vec() -> Vec<Entity> {
    let base_path = "data";
    let paths = fs::read_dir(base_path).unwrap();

    let mut entity_vec = vec![];

    for path in paths {
        let dir_entity = path.unwrap();
        let file_name_os_str = dir_entity.file_name();
        let file_name = file_name_os_str.to_str().unwrap().to_string();
        let file_suffix = file_name.split('.').last().unwrap();
        let entity = Entity {
            to_email_address: file_name
                .clone()
                .trim_end_matches(&format!(".{}", file_suffix))
                .to_string(),
            file_path: format!("{}/{}", base_path, file_name),
        };
        entity_vec.push(entity);
    }

    entity_vec
}

fn main() {
    let file = File::open("email_config.json").unwrap();
    let reader = BufReader::new(file);
    let email_config: EmailConfig = serde_json::from_reader(reader).unwrap();
    println!("email_config: {:#?}", email_config);

    let mut mailer = SmtpClient::new_simple(&email_config.smtp_domain)
        .unwrap()
        .credentials(Credentials::new(
            email_config.from_email_address.clone(),
            email_config.from_email_password,
        ))
        .transport();

    let entity_vec = get_entity_vec();
    println!("entity_vec: {:#?}", entity_vec);

    let mut email_vec = vec![];
    for entity in entity_vec {
        let email = EmailBuilder::new()
            .to(entity.to_email_address)
            .from(email_config.from_email_address.clone())
            .subject(email_config.subject.clone())
            .html(email_config.html_body.clone())
            .attachment_from_file(
                Path::new(&entity.file_path),
                Some(&email_config.attachment_file_name),
                &mime::APPLICATION_OCTET_STREAM,
            )
            .unwrap()
            .build()
            .unwrap();
        email_vec.push(email);
    }

    for email in email_vec {
        let result = mailer.send(email.into());
        println!("{:?}", result);
    }
}

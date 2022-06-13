use lettre::{
    message::{Attachment, Body, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::BufReader,
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

    let creds = Credentials::new(
        email_config.from_email_address.clone(),
        email_config.from_email_password.clone(),
    );

    let mailer = SmtpTransport::relay(&email_config.smtp_domain)
        .unwrap()
        .credentials(creds)
        .build();

    let entity_vec = get_entity_vec();
    println!("entity_vec: {:#?}", entity_vec);

    let email_vec: Vec<Message> = entity_vec
        .iter()
        .map(|entity| {
            let attachment_file = fs::read(entity.file_path.clone()).unwrap();
            let attachment_file_body = Body::new(attachment_file);
            Message::builder()
                .from(email_config.from_email_address.clone().parse().unwrap())
                .to(entity.to_email_address.parse().unwrap())
                .subject(email_config.subject.clone())
                .multipart(
                    MultiPart::mixed()
                        .multipart(
                            MultiPart::alternative()
                                // .singlepart(SinglePart::plain(String::from("Hello, world! :)")))
                                .multipart(
                                    MultiPart::related().singlepart(SinglePart::html(
                                        email_config.html_body.clone(),
                                    )),
                                ),
                        )
                        .singlepart(
                            Attachment::new(String::from(&email_config.attachment_file_name)).body(
                                attachment_file_body,
                                "application/octet-stream".parse().unwrap(),
                            ),
                        ),
                )
                .unwrap()
        })
        .collect();
    println!("email_vec: {:#?}", email_vec);

    for email in email_vec {
        let result = mailer.send(&email);
        println!("{:#?}", result);
    }
}

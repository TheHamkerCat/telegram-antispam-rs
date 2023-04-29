#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;
extern crate bayespam;

mod spam_check;
mod training;

use rocket::Config;


#[rocket::main]
async fn main() {
    println!("Training model...");

    // Train the model and save it to model.json
    // CSV format should be as follows:
    // text_type,text
    // spam,please subscribe to my youtube channel
    // ham,blazingly fast rust
    training::train_model(
        std::path::Path::new("dataset.csv"),
        std::path::Path::new("model.json"),
    ).unwrap();

    // Config for 0.0.0.0:8000 production
    let config = Config {
            address: std::net::Ipv4Addr::new(0, 0, 0, 0).into(),
            port: 8000,
            ..Config::release_default()
        };

    // Launch rocket server
    rocket::custom(&config)
        .attach(spam_check::stage())
        .launch().await.unwrap();
}
extern crate mysql;
extern crate serde;
extern crate rocket;
use backend::MySqlBackend;
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket_firebase_auth::FirebaseAuth;
use slog::o;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use alohomora::rocket::BBoxRocket;
use alohomora_derive::routes;

mod login;
mod backend;
mod common;
mod config;
mod student;
mod instructor;
mod register;
mod admin;
mod policies;

pub fn new_logger() -> slog::Logger {
    use slog::Drain;
    use slog::Logger;
    use slog_term::term_full;
    Logger::root(Mutex::new(term_full()).fuse(), o!())
}

#[rocket::main]
async fn main() {
    let firebase_auth: FirebaseAuth = FirebaseAuth::builder()
        .json_file("src/firebase-credentials.json")
        .build()
        .expect("Failed to read firebase credentials");

    // Initialize the backend
    let config_path = "config.toml";
    let config = config::parse(config_path).unwrap();
    let db_name: &str = "users";
    let backend: Arc<Mutex<MySqlBackend>> = Arc::new(Mutex::new(
    backend::MySqlBackend::new(
        &config.db_user,
        &config.db_password,
        &format!("{}", db_name),
        Some(new_logger()),
        config.prime,
        ).unwrap(),));

    // Setup cors
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            ["Get", "Post", "Put", "Delete", "Options"]
                .iter()
                .map(|s| FromStr::from_str(s).unwrap())
                .collect(),
        )
        .allow_credentials(true)
        .to_cors()
        .expect("Failed to setup cors configuration.");

    // build and launch
    if let Err(e) = BBoxRocket::build()
        //.attach(cors.clone())
        .manage(cors)
        .manage(backend)
        .manage(config)
        .manage(firebase_auth)
        .mount("/", routes![login::login])
        .mount("/", routes![admin::admin])
        .mount("/", routes![student::student, student::update])
        .mount("/", routes![instructor::instructor])
        .mount("/", routes![register::register_instructor, register::register_student])
        // Potential issue?
        //.mount("/", rocket_cors::catch_all_options_routes())
        .launch()
        .await 
    {
        println!("didn't launch properly");
        drop(e);
    };
}

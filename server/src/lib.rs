extern crate mysql;
extern crate rocket;
extern crate serde;
extern crate sesame;
extern crate sesame_mysql;
extern crate sesame_rocket;

use backend::MySqlBackend;
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket_firebase_auth::FirebaseAuth;
use sesame_rocket::rocket::{routes, SesameRocket};
use slog::o;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

mod backend;
mod config;
mod context;
mod policies;
mod routes;

pub fn new_logger() -> slog::Logger {
    use slog::Drain;
    use slog::Logger;
    use slog_term::term_full;
    Logger::root(Mutex::new(term_full()).fuse(), o!())
}

pub fn build_server() -> SesameRocket<rocket::Build> {
    let firebase_auth: FirebaseAuth = FirebaseAuth::builder()
        .json_file("./src/firebase-credentials.json")
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
        )
        .unwrap(),
    ));

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

    // build
    SesameRocket::build()
        .manage(cors.clone())
        .manage(backend)
        .manage(config)
        .manage(firebase_auth)
        // Potential issues?
        .attach(cors.clone())
        .mount("/", sesame_rocket::rocket::catch_all_options_routes())
        .mount("/", routes![routes::login::login])
        .mount("/", routes![routes::admin::admin])
        .mount(
            "/",
            routes![routes::student::student, routes::student::update],
        )
        .mount("/", routes![routes::instructor::instructor])
        .mount(
            "/",
            routes![
                routes::register::register_instructor,
                routes::register::register_student
            ],
        )
}

pub fn build_server_test() -> SesameRocket<rocket::Build> {
    let firebase_auth: FirebaseAuth = FirebaseAuth::builder()
        .json_file("./tests/dummy-firebase-creds.json")
        .jwks_url("http://localhost:8888/jwks_url")
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
        )
        .unwrap(),
    ));

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

    // build
    SesameRocket::build()
        .manage(cors.clone())
        .manage(backend)
        .manage(config)
        .manage(firebase_auth)
        // Potential issues?
        .attach(cors.clone())
        .mount("/", sesame_rocket::rocket::catch_all_options_routes())
        .mount(
            "/",
            routes![
                routes::login::login,
                routes::login::login_email_buggy,
                routes::login::login_auth_buggy
            ],
        )
        .mount("/", routes![routes::admin::admin])
        .mount(
            "/",
            routes![
                routes::student::student,
                routes::student::update,
                routes::student::update_buggy
            ],
        )
        .mount(
            "/",
            routes![
                routes::instructor::instructor,
                routes::instructor::instructor_buggy
            ],
        )
        .mount(
            "/",
            routes![
                routes::register::register_instructor,
                routes::register::register_student,
                routes::register::register_student_buggy
            ],
        )
}

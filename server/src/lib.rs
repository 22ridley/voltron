extern crate mysql;
extern crate rocket;
extern crate serde;
use crate::policies::{ReadBufferPolicy, EmailPolicy};
use alohomora::rocket::{routes, BBoxRocket};
use backend::MySqlBackend;
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket_firebase_auth::FirebaseAuth;
use slog::o;
use std::str::FromStr;
use std::sync::{Arc, Mutex, Once};

mod admin;
mod backend;
mod common;
mod config;
mod context;
mod instructor;
mod login;
mod policies;
mod register;
mod student;

static INIT: Once = Once::new();

pub fn initialize() {
    INIT.call_once(|| {
        // Register all policies. #[schema_policy(...)] does not work on mac.
        // Email in user table
        alohomora::policy::add_schema_policy::<EmailPolicy>(String::from("user"), 2);
        // Class_id in enroll table
        alohomora::policy::add_schema_policy::<ReadBufferPolicy>(String::from("enroll"), 1);
        // Group_id in enroll table
        alohomora::policy::add_schema_policy::<ReadBufferPolicy>(String::from("enroll"), 2);
    });
}

pub fn new_logger() -> slog::Logger {
    use slog::Drain;
    use slog::Logger;
    use slog_term::term_full;
    Logger::root(Mutex::new(term_full()).fuse(), o!())
}

pub fn build_server() -> BBoxRocket<rocket::Build> {
    let firebase_auth: FirebaseAuth = FirebaseAuth::builder()
        .json_file("./src/firebase-credentials.json")
        .build()
        .expect("Failed to read firebase credentials");

    // Register all policies. #[schema_policy(...)] does not work on mac.
    alohomora::policy::add_schema_policy::<EmailPolicy>(String::from("user"), 2);
    alohomora::policy::add_schema_policy::<ReadBufferPolicy>(String::from("enroll"), 1);
    alohomora::policy::add_schema_policy::<ReadBufferPolicy>(String::from("enroll"), 2);

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
    BBoxRocket::build()
        .manage(cors.clone())
        .manage(backend)
        .manage(config)
        .manage(firebase_auth)
        // Potential issues?
        .attach(cors.clone())
        .mount("/", alohomora::rocket::catch_all_options_routes())
        .mount("/", routes![login::login])
        .mount("/", routes![admin::admin])
        .mount("/", routes![student::student, student::update])
        .mount("/", routes![instructor::instructor])
        .mount(
            "/",
            routes![register::register_instructor, register::register_student],
        )
}

pub fn build_server_test() -> BBoxRocket<rocket::Build> {
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
    BBoxRocket::build()
        .manage(cors.clone())
        .manage(backend)
        .manage(config)
        .manage(firebase_auth)
        // Potential issues?
        .attach(cors.clone())
        .mount("/", alohomora::rocket::catch_all_options_routes())
        .mount("/", routes![login::login, login::login_email_buggy, login::login_auth_buggy])
        .mount("/", routes![admin::admin])
        .mount("/", routes![student::student, student::update, student::update_buggy])
        .mount("/", routes![instructor::instructor, instructor::instructor_buggy])
        .mount(
            "/",
            routes![register::register_instructor, register::register_student, register::register_student_buggy],
        )
}
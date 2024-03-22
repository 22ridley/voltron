extern crate mysql;
extern crate serde;
#[macro_use]
extern crate rocket;
use rocket::{Build, Rocket};
extern crate rocket_dyn_templates;
use backend::MySQLBackend;
use common::{AnyResponse, LoginContext};
use mysql::Row;
use rocket::{response::Redirect, State};
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket_dyn_templates::Template;
use rocket_firebase_auth::FirebaseAuth;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

mod api;
mod backend;
mod common;
mod config;
mod instructor;
mod register;
mod student;
mod admin;

#[rocket::launch]
async fn rocket() -> Rocket<Build> {
    let firebase_auth: FirebaseAuth = FirebaseAuth::builder()
        .json_file("firebase-credentials.json")
        .build()
        .expect("Failed to read firebase credentials");

    println!("CREDS:");
    println!("{:?}", firebase_auth);

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

    rocket::build()
        .mount("/", api::routes())
        .mount("/", rocket_cors::catch_all_options_routes())
        .attach(cors.clone())
        .manage(cors)
        .manage(firebase_auth)
}

// #[rocket::main]
// async fn main() {
//     // Get the config file
//     let config_path = "config.toml";
//     let config = config::parse(config_path).unwrap();

//     // Make the template
//     let template_dir: String = config.template_dir.clone();
//     let template = Template::custom(move |engines| {
//         engines
//             .handlebars
//             .register_templates_directory(".hbs", std::path::Path::new(&template_dir))
//             .expect("failed to set template path!");
//     });

//     // Initialize the backend
//     let db_name: &str = "users";
//     let backend: Arc<Mutex<MySQLBackend>> = Arc::new(Mutex::new(
//         backend::MySQLBackend::new(
//             &config.db_user,
//             &config.db_password,
//             &format!("{}", db_name),
//             config.prime,
//         )
//         .unwrap(),
//     ));

//     let firebase_auth = FirebaseAuth::builder()
//         .json_file("firebase-credentials.json")
//         .build()
//         .unwrap();

//     // Setup cors
//     let cors = CorsOptions::default()
//         .allowed_origins(AllowedOrigins::all())
//         .allowed_methods(
//             ["Get", "Post", "Put", "Delete", "Options"]
//                 .iter()
//                 .map(|s| FromStr::from_str(s).unwrap())
//                 .collect(),
//         )
//         .allow_credentials(true)
//         .to_cors()
//         .expect("Failed to setup cors configuration.");

//     // build and launch
//     if let Err(e) = rocket::build()
//         .attach(template)
//         .manage(backend)
//         .manage(config)
//         .manage(AuthState {
//             auth: firebase_auth,
//         })
//         .mount("/", routes![index])
//         .mount("/", rocket_cors::catch_all_options_routes())
//         .attach(cors.clone())
//         .manage(cors)
//         .mount("/login", routes![login])
//         .mount("/view", routes![view])
//         .mount("/admin", routes![admin::admin])
//         .mount("/instructor", routes![instructor::instructor])
//         .mount("/student", routes![student::student])
//         .mount("/update", routes![student::update])
//         .mount(
//             "/register-instructor",
//             routes![register::register_instructor],
//         )
//         .mount("/register-student", routes![register::register_student])
//         .launch()
//         .await
//     {
//         println!("Didn't launch properly");
//         drop(e);
//     };
// }

// Index redirects to login
#[get("/")]
fn index() -> Redirect {
    Redirect::to("/login")
}

// Login page directs instructor to class view
// Login page directs students to student view
// Grouped students see the same student view
#[get("/?<fail>")]
pub fn login(fail: Option<&str>) -> Template {
    let mut retry: bool = false;
    if fail.is_some() {
        retry = true;
    }
    let ctx: LoginContext = LoginContext {
        failed: retry
    };
    Template::render("login", ctx)
}

#[get("/?<name>")]
pub fn view(name: &str, backend: &State<Arc<Mutex<MySQLBackend>>>) 
 -> AnyResponse {
    let mut bg: std::sync::MutexGuard<'_, MySQLBackend> = backend.lock().unwrap();
    let user_res: Vec<Row> = (*bg).prep_exec("SELECT * FROM users WHERE user_name = ?", vec![name]).unwrap();
    drop(bg);
    if user_res.len() == 0 { return AnyResponse::Redirect(Redirect::to("/login?fail")); }
    let row: Row = user_res.get(0).unwrap().clone();
    let privilege: Option<i32> =  row.get(1).unwrap();
    let class_id: Option<i32> = row.get(2).unwrap();
    let group_id: Option<i32> = row.get(3).unwrap();
    let privl: i32 = privilege.unwrap();
    let name_arg: String = name.replace(" ", "+");
    if privl == 2 {
        AnyResponse::Redirect(Redirect::to(format!("/admin")))
    } else if privl == 1 {
        AnyResponse::Redirect(Redirect::to(format!("/instructor?name={}&class_id={}", name_arg, class_id.unwrap())))
    } else {
        AnyResponse::Redirect(Redirect::to(format!("/student?name={}&class_id={}&group_id={}", name_arg, class_id.unwrap(), group_id.unwrap())))
    }
}
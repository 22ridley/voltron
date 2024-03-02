extern crate serde;
extern crate mysql;
#[macro_use]
extern crate rocket;
extern crate rocket_dyn_templates;
use std::{sync::Arc, sync::Mutex};
use mysql::{prelude::Queryable, Row};
use backend::MySQLBackend;
use rocket::{response::Redirect, State};
use rocket_dyn_templates::Template;

mod config;
mod common;
mod backend;

#[rocket::main]
async fn main() {
    // Get the config file
    let config_path = "config.toml";
    let config = config::parse(config_path).unwrap();

    // Make the template
    let template_dir = config.template_dir.clone();
    let template = Template::custom(move |engines| {
        engines
            .handlebars
            .register_templates_directory(".hbs", std::path::Path::new(&template_dir))
            .expect("failed to set template path!");
    });

    // Initialize the backend
    let db_name = "users";
    let backend = Arc::new(Mutex::new(
        backend::MySQLBackend::new(
            &config.db_user,
            &config.db_password,
            &format!("{}", db_name),
            config.prime
        ).unwrap()
    ));

    // build and launch
    if let Err(e) = rocket::build()
        .attach(template)
        .manage(backend)
        .manage(config)
        .mount("/", routes![index])
        .mount("/login", routes![login])
        .mount("/view", routes![view])
        .mount("/instructor", routes![instructor])
        .mount("/student", routes![student])
        .launch()
        .await 
    {
        println!("Didn't launch properly");
        drop(e);
    };
}

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
    let ctx = common::LoginContext {
        failed: retry
    };
    Template::render("login", ctx)
}

#[get("/?<name>")]
pub fn view(name: &str, backend: &State<Arc<Mutex<MySQLBackend>>>) 
-> common::AnyResponse {
    let mut bg = backend.lock().unwrap();
    let user_res: Vec<Row> = (*bg).handle.query(format!("SELECT * FROM users WHERE user_name = \"{}\"", name)).unwrap();
    drop(bg);
    if user_res.len() == 0 { return common::AnyResponse::Redirect(Redirect::to("/login?fail")); }
    let row: Row = user_res.get(0).unwrap().clone();
    let row_name: Option<i32> =  row.get(1).unwrap();
    if row_name.unwrap() != 0 {
        common::AnyResponse::Redirect(Redirect::to(format!("/instructor/{}", name)))
    } else {
        common::AnyResponse::Redirect(Redirect::to(format!("/student/{}", name)))
    }
}

#[get("/<name>")]
pub fn instructor(name: &str) -> common::AnyResponse {
    // Get the user information from the backend database
    let mut is_admin: bool = false;
    if name == "admin" {
        is_admin = true;
    }

    // Create the context for the template
    let ctx = common::InstructorContext {
        name: name.to_string(),
        admin: is_admin
    };
    common::AnyResponse::Template(Template::render("instructor", &ctx))
}

#[get("/<name>")]
pub fn student(name: &str) -> Template {
    let ctx = common::StudentContext {
        name: name.to_string()
    };
    Template::render("student", &ctx)
}
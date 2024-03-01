extern crate serde;
extern crate mysql;
#[macro_use]
extern crate rocket;
extern crate rocket_dyn_templates;
use std::collections::HashMap;
use rocket::response::Redirect;
use rocket_dyn_templates::Template;

mod config;
mod common;

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

    // build and launch
    if let Err(e) = rocket::build()
        .attach(template)
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
#[get("/")]
pub fn login() -> Template {
    let ctx: HashMap<&str, &str> = HashMap::new();
    Template::render("login", ctx)
}

#[get("/?<name>")]
pub fn view(name: &str) -> Redirect {
    println!("{name}");
    //Redirect::to("/login")
    Redirect::to(format!("/instructor/{}", name))
}

#[get("/<name>")]
pub fn instructor(name: &str) -> Template {
    let ctx = common::InstructorContext {
        name: name.to_string()
    };
    Template::render("instructor", &ctx)
}

#[get("/<name>")]
pub fn student(name: &str) -> Template {
    let ctx = common::StudentContext {
        name: name.to_string()
    };
    Template::render("student", &ctx)
}
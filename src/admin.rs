use crate::common::{AdminContext, AnyResponse, Instructor};
use crate::backend::MySQLBackend;
use rocket::State;
use rocket_dyn_templates::Template;
use std::sync::{Arc, Mutex};

#[get("/?<reg_name>&<reg_class>")]
pub fn admin(reg_name: Option<&str>, reg_class: Option<&str>, 
    backend: &State<Arc<Mutex<MySQLBackend>>>) -> AnyResponse {
    // Get the user information from the backend database
    let mut register_name: &str = "";
    let mut register_class: &str = "";
    if reg_name.is_some() & reg_class.is_some() {
        register_class = reg_class.unwrap();
        register_name = reg_name.unwrap();
    }

    // Get list of all students
    let mut bg: std::sync::MutexGuard<'_, MySQLBackend> = backend.lock().unwrap();
    let instructors_res: Vec<Instructor> = (*bg).prep_exec("SELECT * FROM users WHERE privilege = 1", ()).unwrap();
    drop(bg);

    // Create the context for the template
    let ctx: AdminContext = AdminContext {
        instructors: instructors_res,
        registered_name: register_name.to_string(),
        registered_class: register_class.to_string(),
    };
    AnyResponse::Template(Template::render("admin", &ctx))
}
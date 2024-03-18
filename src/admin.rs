use crate::common::{AdminContext, AnyResponse, Instructor};
use crate::backend::MySQLBackend;
use rocket::State;
use rocket_dyn_templates::Template;
use std::sync::{Arc, Mutex};

#[get("/?<fail_message>")]
pub fn admin(fail_message: Option<&str>, 
    backend: &State<Arc<Mutex<MySQLBackend>>>) -> AnyResponse {
    // Get the user information from the backend database
    let mut fail: bool = false;
    let mut fail_string: &str = "";
    if fail_message.is_some() {
        fail = true;
        fail_string = fail_message.unwrap();
    }

    // Get list of all students
    let mut bg: std::sync::MutexGuard<'_, MySQLBackend> = backend.lock().unwrap();
    let instructors_res: Vec<Instructor> = (*bg).prep_exec("SELECT * FROM users WHERE privilege = 1", ()).unwrap();
    drop(bg);

    // Create the context for the template
    let ctx: AdminContext = AdminContext {
        instructors: instructors_res,
        fail: fail,
        fail_message: fail_string.to_string(),
    };
    AnyResponse::Template(Template::render("admin", &ctx))
}
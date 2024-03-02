use mysql::serde::Serialize;
use rocket::response::Redirect;
use rocket_dyn_templates::Template;

// Response enum that can be either templates or redirects
#[derive(Debug, Responder)]
pub enum AnyResponse {
    Template(Template),
    Redirect(Redirect),
}

// The context needed for rendering the login page
#[derive(Serialize)]
pub struct LoginContext {
    pub failed: bool
}

// The context needed for rendering the instructor page
#[derive(Serialize)]
pub struct InstructorContext {
    pub name: String,
    pub admin: bool,
    pub message_cntnt: String,
    pub message: bool
}

// The context needed for rendering the student page
#[derive(Serialize)]
pub struct StudentContext {
    pub name: String
}
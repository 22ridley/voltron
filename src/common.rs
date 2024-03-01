use mysql::serde::Serialize;

// The context needed for rendering the instructor page
#[derive(Serialize)]
pub struct InstructorContext {
    pub name: String
}

// The context needed for rendering the student page
#[derive(Serialize)]
pub struct StudentContext {
    pub name: String
}
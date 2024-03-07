use mysql::serde::Serialize;
use rocket::response::Redirect;
use rocket_dyn_templates::Template;
use mysql::Row;
use mysql::prelude::FromRow;
use mysql::from_value;

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

// The structure representing student groups and their code
#[derive(Serialize)]
pub struct StudentGroup{
    pub group_id: i32,
    pub code: String,
    pub index: usize
}

impl StudentGroup{
    pub fn new(row: Row, index: usize) -> Self {
        StudentGroup{
            group_id: from_value(row[0].clone()),
            code: from_value(row[1].clone()),
            index: index
        }
    } 
}

impl FromRow for StudentGroup {
    fn from_row_opt(row: Row) -> Result<Self, mysql::FromRowError> {
        Ok(StudentGroup::new(row, 0))
    }

    fn from_row(row: Row) -> Self {
        StudentGroup::new(row, 0)
    }
}


// The structure representing students
#[derive(Serialize)]
pub struct Student{
    pub name: String,
    pub group_id: i32,
    pub index: usize
}

impl Student{
    pub fn new(row: Row, index: usize) -> Self {
        Student{
            name: from_value(row[0].clone()),
            group_id: from_value(row[2].clone()),
            index: index
        }
    } 
}

impl FromRow for Student {
    fn from_row_opt(row: Row) -> Result<Self, mysql::FromRowError> {
        Ok(Student::new(row, 0))
    }

    fn from_row(row: Row) -> Self {
        Student::new(row, 0)
    }
}

// The context needed for rendering the instructor page
#[derive(Serialize)]
pub struct InstructorContext {
    pub name: String,
    pub admin: bool,
    pub registered_name: String,
    pub registered_instructor: bool,
    pub registered_student: bool,
    pub students: Vec<Student>,
    pub student_groups: Vec<StudentGroup>
}

// The context needed for rendering the student page
#[derive(Serialize)]
pub struct StudentContext {
    pub name: String,
    pub group_id: i32,
    pub text: String
}

#[derive(FromForm)]
pub struct RegisterRequest{
    pub(crate) registrar_name: String,
    pub(crate) registrant_name: String
}

#[derive(FromForm)]
pub struct UpdateRequest{
    pub(crate) name: String,
    pub(crate) group_id: i32,
    pub(crate) text: String
}
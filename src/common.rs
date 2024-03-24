use mysql::serde::Serialize;
use rocket::http::{ContentType, Status};
use rocket::response::{self, Redirect};
use rocket::serde::json::Json;
use rocket::{Request, Response};
use rocket_dyn_templates::Template;
use mysql::Row;
use mysql::prelude::FromRow;
use mysql::from_value;

/// The struct we return for success responses (200s)
#[derive(Debug)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    pub json: Option<Json<T>>,
    pub status: Status,
}

#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    pub success: bool,
}

/// Implements the `Responder` trait for Rocket, so we can simply return a for
/// endpoint functions, result and Rocket takes care of the rest.
impl<'r, T: Serialize> response::Responder<'r, 'r> for ApiResponse<T> {
    fn respond_to(self, req: &'r Request) -> response::Result<'r> {
        Response::build_from(self.json.respond_to(req)?)
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

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

// The structure representing instructors
#[derive(Serialize)]
pub struct Instructor{
    pub name: String,
    pub class_id: i32,
    pub index: usize
}

impl Instructor{
    pub fn new(row: Row, index: usize) -> Self {
        Instructor{
            name: from_value(row[0].clone()),
            class_id: from_value(row[2].clone()),
            index: index
        }
    } 
}

impl FromRow for Instructor {
    fn from_row_opt(row: Row) -> Result<Self, mysql::FromRowError> {
        Ok(Instructor::new(row, 0))
    }

    fn from_row(row: Row) -> Self {
        Instructor::new(row, 0)
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
            group_id: from_value(row[3].clone()),
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

// The context needed for rendering the admin page
#[derive(Serialize)]
pub struct AdminContext {
    pub instructors: Vec<Instructor>,
    pub fail: bool,
    pub fail_message: String
}

// The context needed for rendering the instructor page
#[derive(Serialize)]
pub struct InstructorContext {
    pub name: String,
    pub class_id: String,
    pub fail: bool,
    pub fail_message: String,
    pub students: Vec<Student>,
    pub student_groups: Vec<StudentGroup>
}

// The context needed for rendering the student page
#[derive(Serialize)]
pub struct StudentContext {
    pub name: String,
    pub class_id: i32,
    pub group_id: i32,
    pub text: String
}

#[derive(FromForm)]
pub struct RegisterInstructorRequest{
    pub(crate) instructor_name: String,
    pub(crate) class_id: String
}

#[derive(FromForm)]
pub struct RegisterStudentRequest {
    pub(crate) instructor_name: String,
    pub(crate) student_name: String,
    pub(crate) class_id: String,
    pub(crate) group_id: String,
}

#[derive(FromForm)]
pub struct UpdateRequest{
    pub(crate) name: String,
    pub(crate) class_id: i32,
    pub(crate) group_id: i32,
    pub(crate) text: String
}
extern crate serde;
extern crate mysql;
use std::path::Path;
use std::{sync::Arc, sync::Mutex, cmp};
use mysql::Row;
use rocket::{response::Redirect, State, form::Form};
use crate::backend::MySQLBackend;
use crate::common::{AnyResponse, RegisterStudentRequest, RegisterInstructorRequest};
use std::fs::File;

#[post("/", data="<data>")]
pub fn register_instructor(data: Form<RegisterInstructorRequest>, 
    backend: &State<Arc<Mutex<MySQLBackend>>>) -> AnyResponse {
    let instructor_name = &data.instructor_name.clone();
    let class_id = &data.class_id.clone();
    // Assemble values to insert
    let users_row: Vec<&str> = vec![instructor_name, "1", class_id, "-1"];

    // Make insert query to add this new instructor
    let q = "INSERT INTO users (user_name, privilege, class_id, group_id) VALUES (?, ?, ?, ?)";

    // send insert query to db
    let mut bg = backend.lock().unwrap();
    let _res: Vec<Row> = (*bg).prep_exec(q, users_row).unwrap();
    drop(bg);

    AnyResponse::Redirect(Redirect::to(format!("/admin?reg_name={}&reg_class={}", instructor_name, class_id)))
}

#[post("/", data="<data>")]
pub fn register_student(data: Form<RegisterStudentRequest>,
    backend: &State<Arc<Mutex<MySQLBackend>>>)-> AnyResponse {
    // Count the number of students currently in the database 
    let mut bg = backend.lock().unwrap();
    let student_group: &str = &data.group_id.to_string();
    let student_name = &data.student_name.clone();
    let student_class = &data.class_id.clone();
    let instructor_name = &data.instructor_name.clone();
    let users_row: Vec<&str> = vec![student_name, "0", student_class, student_group];
    // If this group ID is new, create a new file
    let file_string: String = format!("group_code/group{}_code.txt", student_group);
    let file_name: &Path = Path::new(&file_string);
    if !file_name.is_file() {
        // Open a new file for group_id max_student_group + 1
        let _ = File::create(file_name);
    }   

    // Make insert query to add this new student into users
    let q: &str = "INSERT INTO users (user_name, privilege, class_id, group_id) VALUES (?, ?, ?, ?)";
    let _res: Vec<Row> = (*bg).prep_exec(q, users_row).unwrap();
    drop(bg);
    AnyResponse::Redirect(Redirect::to(format!("/instructor?name={}&class_id={}&reg_name={}&reg_type={}", instructor_name, student_class, student_name, "stud")))
}
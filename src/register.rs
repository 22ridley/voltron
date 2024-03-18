extern crate serde;
extern crate mysql;
use std::path::Path;
use std::{sync::Arc, sync::Mutex};
use mysql::Row;
use rocket::{response::Redirect, State, form::Form};
use crate::backend::MySQLBackend;
use crate::common::{AnyResponse, RegisterStudentRequest, RegisterInstructorRequest};
use std::fs::File;

#[post("/", data="<data>")]
pub fn register_instructor(data: Form<RegisterInstructorRequest>, 
    backend: &State<Arc<Mutex<MySQLBackend>>>) -> AnyResponse {
    let instructor_name: &String = &data.instructor_name.clone();
    let class_id: &String = &data.class_id.clone();
    let mut fail: bool = false;
    let mut fail_message: String = "".to_string();
    match class_id.parse::<i32>() {
        Ok(_i32) => (),
        Err(_e) => {
            fail = true;
            let with_spaces = format!("Class ID '{}' could not be parsed into an integer", class_id);
            fail_message = with_spaces.replace(" ", "+");
        }
    }
    if fail {
        return AnyResponse::Redirect(Redirect::to(format!("/admin?fail_message={}", fail_message)))
    } 
    // Assemble values to insert
    let users_row: Vec<&str> = vec![instructor_name, "1", class_id, "-1"];

    // Make insert query to add this new instructor
    let q = "INSERT INTO users (user_name, privilege, class_id, group_id) VALUES (?, ?, ?, ?)";

    // send insert query to db
    let mut bg = backend.lock().unwrap();
    let _res: Vec<Row> = (*bg).prep_exec(q, users_row).unwrap();
    drop(bg);

    AnyResponse::Redirect(Redirect::to(format!("/admin")))
}

#[post("/", data="<data>")]
pub fn register_student(data: Form<RegisterStudentRequest>,
    backend: &State<Arc<Mutex<MySQLBackend>>>)-> AnyResponse {
    // Count the number of students currently in the database 
    let mut bg = backend.lock().unwrap();
    let student_group: &String = &data.group_id.clone();
    let student_name: &String = &data.student_name.clone();
    let student_class: &String = &data.class_id.clone();
    let instructor_name: &String = &data.instructor_name.clone();
    // Check if student_group is numeric
    let mut fail: bool = false;
    let mut fail_message: String = "".to_string();
    match student_group.parse::<i32>() {
        Ok(_i32) => (),
        Err(_e) => {
            fail = true;
            let with_spaces = format!("Group ID '{}' could not be parsed into an integer", student_group);
            fail_message = with_spaces.replace(" ", "+");
        }
    }
    if fail {
        return AnyResponse::Redirect(Redirect::to(format!("/instructor?name={}&class_id={}&fail_message={}", instructor_name, student_class, fail_message)))
    }

    let users_row: Vec<&str> = vec![student_name, "0", student_class, student_group];
    // If this group ID is new, create a new file
    let file_string: String = format!("group_code/class{}_group{}_code.txt", student_class, student_group);
    let file_name: &Path = Path::new(&file_string);
    if !file_name.is_file() {
        // Open a new file for group_id max_student_group + 1
        let _ = File::create(file_name);
    }   

    // Make insert query to add this new student into users
    let q: &str = "INSERT INTO users (user_name, privilege, class_id, group_id) VALUES (?, ?, ?, ?)";
    let _res: Vec<Row> = (*bg).prep_exec(q, users_row).unwrap();
    drop(bg);
    AnyResponse::Redirect(Redirect::to(format!("/instructor?name={}&class_id={}", instructor_name, student_class)))
}
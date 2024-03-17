extern crate serde;
extern crate mysql;
use std::{sync::Arc, sync::Mutex, cmp};
use mysql::Row;
use rocket::{response::Redirect, State, form::Form};
use crate::backend::MySQLBackend;
use crate::common::{AnyResponse, RegisterRequest};
use std::fs::File;

#[post("/", data="<data>")]
pub fn register_instructor(data: Form<RegisterRequest>, 
    backend: &State<Arc<Mutex<MySQLBackend>>>) -> AnyResponse {
    let instructor_name = &data.registrant_name.clone();
    // Assemble values to insert
    let users_row: Vec<&str> = vec![instructor_name, "1", "0", "-1"];

    // Make insert query to add this new instructor
    let q = "INSERT INTO users (user_name, privilege, class_id, group_id) VALUES (?, ?, ?, ?)";

    // send insert query to db
    let mut bg = backend.lock().unwrap();
    let _res: Vec<Row> = (*bg).prep_exec(q, users_row).unwrap();
    drop(bg);

    AnyResponse::Redirect(Redirect::to(format!("/instructor?name=admin&reg_name={}&reg_type={}", instructor_name, "inst")))
}

#[post("/", data="<data>")]
pub fn register_student(data: Form<RegisterRequest>,
    backend: &State<Arc<Mutex<MySQLBackend>>>)-> AnyResponse {
    // Count the number of students currently in the database 
    let mut bg = backend.lock().unwrap();
    let students_res: Vec<Row> = (*bg).prep_exec("SELECT * FROM users WHERE privilege = 0", ()).unwrap();    
    let mut max_student_group: i32 = 0;
    for student in students_res.iter() {
        let group_id: i32 = student.clone().get(2).unwrap();
        max_student_group = cmp::max(max_student_group, group_id);
    }
    let mut student_group: i32 = max_student_group;
    // There are an even number of students, so there are no students alone
    if students_res.len() % 2 == 0 {
        // The student will have group_id max_student_group + 1
        student_group += 1;

        // Open a new file for group_id max_student_group + 1
        let path: String = format!("group_code/group{}_code.txt", student_group);
        let _ = File::create(&path);
    }
    // Otherwise, there are an odd number of students, so there is some student alone
    // and the student will have group_id max_student_group
    let student_group_string: &str = &student_group.to_string();
    let student_name = &data.registrant_name.clone();
    let instructor_name = &data.registrar_name.clone();
    let users_row: Vec<&str> = vec![student_name, "0", "0", student_group_string];    

    // Make insert query to add this new student into users
    let q: &str = "INSERT INTO users (user_name, privilege, class_id, group_id) VALUES (?, ?, ?, ?)";
    let _res: Vec<Row> = (*bg).prep_exec(q, users_row).unwrap();
    drop(bg);
    AnyResponse::Redirect(Redirect::to(format!("/instructor?name={}&reg_name={}&reg_type={}", instructor_name, student_name, "stud")))
}
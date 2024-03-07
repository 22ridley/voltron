extern crate serde;
extern crate mysql;
use std::{sync::Arc, sync::Mutex, cmp};
use mysql::{prelude::Queryable, Row};
use rocket::{response::Redirect, State};
use crate::backend::MySQLBackend;
use crate::common::AnyResponse;
use std::fs::File;

#[get("/?<name>")]
pub fn register_instructor(name: &str, backend: &State<Arc<Mutex<MySQLBackend>>>) 
-> AnyResponse {
    // Assemble values to insert
    let users_row: Vec<&str> = vec![name, "1", "-1"];

    // Make insert query to add this new instructor
    let q = format!("INSERT INTO users (user_name, privilege, group_id) VALUES ({})", 
                            users_row.iter().map(|s| {format!("\"{s}\"")})
                                .collect::<Vec<String>>()
                                .join(","));

    // send insert query to db
    let mut bg = backend.lock().unwrap();
    let _ = (*bg).handle.query_drop(q).unwrap();
    drop(bg);

    AnyResponse::Redirect(Redirect::to(format!("/instructor?name=admin&reg_name={}&reg_type={}", name, "inst")))
}

#[get("/?<name>&<student_name>")]
pub fn register_student(name: &str, student_name: &str, 
    backend: &State<Arc<Mutex<MySQLBackend>>>)-> AnyResponse {
    // Count the number of students currently in the database 
    let mut bg = backend.lock().unwrap();
    let students_res: Vec<Row> = (*bg).handle.query(format!("SELECT * FROM users WHERE privilege = 0")).unwrap();    
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
    let users_row: Vec<&str> = vec![student_name, "0", student_group_string];    

    // Make insert query to add this new student into users
    let q = format!("INSERT INTO users (user_name, privilege, group_id) VALUES ({})", 
                            users_row.iter().map(|s| {format!("\"{s}\"")})
                                .collect::<Vec<String>>()
                                .join(","));
    let _ = (*bg).handle.query_drop(q).unwrap();
    drop(bg);
    AnyResponse::Redirect(Redirect::to(format!("/instructor?name={}&reg_name={}&reg_type={}", name, student_name, "stud")))
}
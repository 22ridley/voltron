extern crate serde;
extern crate mysql;
use mysql::Row;
use std::{sync::Arc, sync::Mutex};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::Route;
use rocket::State;
use crate::backend::MySQLBackend;
use crate::common::{ApiResponse, SuccessResponse};
use rocket_firebase_auth::FirebaseToken;

pub fn routes() -> Vec<Route> {
    routes![register_instructor, register_student]
}

#[post("/register_instructor?<instr_name>&<instr_class>&<instr_email>")]
pub fn register_instructor(_token: FirebaseToken, instr_name: Option<&str>,
    instr_class: Option<&str>, instr_email: Option<&str>,
    backend: &State<Arc<Mutex<MySQLBackend>>>) -> ApiResponse<SuccessResponse> {
    let instructor_name: &str = instr_name.unwrap();
    let class_name: &str = instr_class.unwrap();
    let instructor_email: &str = instr_email.unwrap();

    // Make insert query to add this new instructor to user
    let mut bg = backend.lock().unwrap();
    let user_row: Vec<&str> = vec![instructor_name, instructor_email.clone(), "1"];
    let result = (*bg).prep_exec::<_, _, Vec<u8>>(
        "INSERT INTO user (user_name, email, privilege) VALUES (?, ?, ?)",
        user_row
    );
    let new_res: Vec<Row> = (*bg).prep_exec("SELECT user_id FROM user WHERE email = ?", vec![instructor_email.clone()]).unwrap();
    let new_row: Row = new_res.get(0).unwrap().clone();
    let instructor_id: i32 = new_row.get(0).unwrap();
    let instructor_id_binding: String = instructor_id.to_string();
    // Insert into class
    let class_row: Vec<&str> = vec![class_name, &instructor_id_binding];
    let _ = (*bg).prep_exec::<_, _, Vec<u8>>(
        "INSERT INTO class (class_name, instructor_id) VALUES (?, ?)",
        class_row
    );
    drop(bg);

    ApiResponse {
        json: Some(Json(SuccessResponse {
            success: true,
            message: "".to_string()
        })),
        status: Status::Ok,
    }
}

#[post("/register_student?<stud_group>&<stud_name>&<stud_class>&<stud_email>")]
pub fn register_student(_token: FirebaseToken, stud_group: Option<&str>, stud_name: Option<&str>,
    stud_class: Option<&str>, stud_email: Option<&str>, 
    backend: &State<Arc<Mutex<MySQLBackend>>>)-> ApiResponse<SuccessResponse> {
    let student_group: &str = stud_group.unwrap();
    let student_name: &str = stud_name.unwrap();
    let student_class: &str = stud_class.unwrap();
    let student_email: &str = stud_email.unwrap();

    // Make insert query to add this new student
    let mut bg = backend.lock().unwrap();
    let group_row: Vec<&str> = vec![student_group, student_class];
    let result = (*bg).prep_exec::<_, _, Vec<u8>>(
        "INSERT IGNORE INTO `group` (group_name, class_id) VALUES (?, ?)",
        group_row
    );
    // Find the group_id
    let new_res: Vec<Row> = (*bg).prep_exec("SELECT group_id FROM `group` WHERE group_name = ?", vec![student_group.clone()]).unwrap();
    let new_row: Row = new_res.get(0).unwrap().clone();
    let group_id: i32 = new_row.get(0).unwrap();
    let group_id_binding: String = group_id.to_string();
    // Insert into user table
    let users_row: Vec<&str> = vec![student_name, student_email, "0"];
    let result = (*bg).prep_exec::<_, _, Vec<u8>>(
        "INSERT INTO user (user_name, email, privilege) VALUES (?, ?, ?)",
        users_row
    );
     // Find the student_id
    let new_res: Vec<Row> = (*bg).prep_exec("SELECT user_id FROM user WHERE email = ?", vec![student_email.clone()]).unwrap();
    let new_row: Row = new_res.get(0).unwrap().clone();
    let student_id: i32 = new_row.get(0).unwrap();
    let student_id_binding: String = student_id.to_string();
    let enroll_row: Vec<&str> = vec![&student_id_binding, student_class, &group_id_binding];
    // Insert into enroll
    let _ = (*bg).prep_exec::<_, _, Vec<u8>>(
        "INSERT INTO enroll (student_id, class_id, group_id) VALUES (?, ?, ?)",
        enroll_row
    );
    drop(bg);

    ApiResponse {
        json: Some(Json(SuccessResponse {
            success: true,
            message: "".to_string()
        })),
        status: Status::Ok,
    }
}
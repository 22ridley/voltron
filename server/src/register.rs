extern crate serde;
extern crate mysql;
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
    let class_id: &str = instr_class.unwrap();
    let instructor_email: &str = instr_email.unwrap();

    // Make insert query to add this new instructor
    let mut bg = backend.lock().unwrap();
    let users_row: Vec<&str> = vec![instructor_name, instructor_email, "1", class_id, "-1"];
    let _ = (*bg).prep_exec::<_, _, Vec<u8>>(
        "INSERT INTO users (user_name, email, privilege, class_id, group_id) VALUES (?, ?, ?, ?, ?)",
        users_row
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
    let users_row: Vec<&str> = vec![student_name, student_email, "0", student_class, student_group];
    let _ = (*bg).prep_exec::<_, _, Vec<u8>>(
        "INSERT INTO users (user_name, email, privilege, class_id, group_id) VALUES (?, ?, ?, ?, ?)",
        users_row
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
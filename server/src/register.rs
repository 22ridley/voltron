extern crate mysql;
extern crate serde;
use crate::backend::MySqlBackend;
use crate::common::SuccessResponse;
use crate::context::ContextDataType;
use crate::policies::{AuthStatePolicy, InstructorPolicy, StudentPolicy};
use alohomora::context::Context;
use alohomora::rocket::post;
use alohomora::bbox::BBox;
use rocket::serde::json::Json;
use rocket::State;
use rocket_firebase_auth::FirebaseToken;
use std::{sync::Arc, sync::Mutex};

#[post("/register_instructor?<instr_name>&<instr_class>&<instr_email>")]
pub fn register_instructor(
    _token: BBox<FirebaseToken, AuthStatePolicy>,
    instr_name: BBox<String, InstructorPolicy>,
    instr_class: BBox<i32, InstructorPolicy>,
    instr_email: BBox<String, InstructorPolicy>,
    backend: &State<Arc<Mutex<MySqlBackend>>>,
    context: Context<ContextDataType>,
) -> Json<SuccessResponse> {
    // Make insert query to add this new instructor
    let mut bg = backend.lock().unwrap();

    // send insert query to db
    bg.prep_exec(
        "INSERT INTO users (user_name, email, privilege, class_id, group_id) VALUES (?, ?, ?, ?, ?)",
        (instr_name, instr_email, 1i32, instr_class, -1i32),
        context.clone(),
    );
    drop(bg);

    Json(SuccessResponse {
        success: true,
        message: "".to_string(),
    })
}

#[post("/register_student?<stud_group>&<stud_name>&<stud_class>&<stud_email>")]
pub fn register_student(
    _token: BBox<FirebaseToken, AuthStatePolicy>,
    stud_group: BBox<i32, StudentPolicy>,
    stud_name: BBox<String, StudentPolicy>,
    stud_class: BBox<i32, StudentPolicy>,
    stud_email: BBox<String, StudentPolicy>,
    backend: &State<Arc<Mutex<MySqlBackend>>>,
    context: Context<ContextDataType>,
) -> Json<SuccessResponse> {
    let mut bg = backend.lock().unwrap();
    // Make insert query to add this new student into users
    (*bg).prep_exec(
        "INSERT INTO users (user_name, email, privilege, class_id, group_id) VALUES (?, ?, ?, ?, ?)", 
        (stud_name, stud_email, 0i32, stud_class.clone(), stud_group.clone()), 
        context.clone()
    );
    drop(bg);

    return Json(SuccessResponse {
        success: true,
        message: "".to_string(),
    });
}

// Buggy version of endpoint!
#[post("/register_student_buggy?<stud_group>&<stud_name>&<stud_class>&<stud_email>")]
pub fn register_student_buggy(
    _token: BBox<FirebaseToken, AuthStatePolicy>,
    stud_group: BBox<i32, StudentPolicy>,
    stud_name: BBox<String, StudentPolicy>,
    stud_class: BBox<i32, StudentPolicy>,
    stud_email: BBox<String, StudentPolicy>,
    backend: &State<Arc<Mutex<MySqlBackend>>>,
    context: Context<ContextDataType>,
) -> Json<SuccessResponse> {
    let mut bg = backend.lock().unwrap();
    // BUGGY: Make insert query to add this new student into users
    // Hard-coded to always insert to class 2!! This should fail student policy
    (*bg).prep_exec(
        "INSERT INTO users (user_name, email, privilege, class_id, group_id) VALUES (?, ?, ?, ?, ?)", 
        (stud_name, stud_email, 0i32, 2i32, stud_group.clone()), 
        context.clone()
    );
    drop(bg);

    return Json(SuccessResponse {
        success: true,
        message: "".to_string(),
    });
}

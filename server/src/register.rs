extern crate mysql;
extern crate serde;
use mysql::Value;
use crate::backend::MySqlBackend;
use crate::common::SuccessResponse;
use crate::context::ContextDataType;
use crate::policies::{AuthStatePolicy, InstructorPolicy, StudentPolicy};
use alohomora::context::Context;
use alohomora::rocket::post;
use alohomora::bbox::BBox;
use alohomora::policy::AnyPolicy;
use alohomora::db::from_value;
use rocket::serde::json::Json;
use rocket::State;
use rocket_firebase_auth::FirebaseToken;
use std::{sync::Arc, sync::Mutex};

#[post("/register_instructor?<instr_name>&<instr_class>&<instr_email>")]
pub fn register_instructor(
    _token: BBox<FirebaseToken, AuthStatePolicy>,
    instr_name: BBox<String, InstructorPolicy>,
    instr_class: BBox<String, InstructorPolicy>,
    instr_email: BBox<String, InstructorPolicy>,
    backend: &State<Arc<Mutex<MySqlBackend>>>,
    context: Context<ContextDataType>,
) -> Json<SuccessResponse> {
    // Make insert query to add this new instructor
    let mut bg = backend.lock().unwrap();

    // send insert query to db
    let _ = bg.prep_exec(
        "INSERT INTO user (user_name, email, privilege) VALUES (?, ?, ?)",
        (instr_name, instr_email.clone(), 1i32),
        context.clone()
    );

    // Find newest inserted user_id
    let new_res: Vec<Vec<BBox<Value, AnyPolicy>>> = bg.prep_exec("SELECT LAST_INSERT_ID()", 
        (), 
        context.clone());
    let instructor_id_val: BBox<Value, AnyPolicy> = new_res[0][0].clone();
    let instructor_id: BBox<i32, AnyPolicy> = from_value(instructor_id_val).unwrap();
    let _ = bg.prep_exec(
        "INSERT INTO class (class_name, instructor_id) VALUES (?, ?)",
        (instr_class, instructor_id),
        context.clone()
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
    stud_group: BBox<String, StudentPolicy>,
    stud_name: BBox<String, StudentPolicy>,
    stud_class: BBox<i32, StudentPolicy>,
    stud_email: BBox<String, StudentPolicy>,
    backend: &State<Arc<Mutex<MySqlBackend>>>,
    context: Context<ContextDataType>,
) -> Json<SuccessResponse> {
    let mut bg = backend.lock().unwrap();
    // Make insert query to add this new student into users
    (*bg).prep_exec(
        "INSERT IGNORE INTO `group` (group_name, class_id) VALUES (?, ?) 
        ON DUPLICATE KEY UPDATE group_id = LAST_INSERT_ID(group_id)", 
        (stud_group.clone(), stud_class.clone()), 
        context.clone()
    );
    // Find the group_id
    let new_res: Vec<Vec<BBox<Value, AnyPolicy>>> = (*bg).prep_exec(
        "SELECT LAST_INSERT_ID()", 
        (),
        context.clone());
    let group_id_val: BBox<Value, AnyPolicy> = new_res[0][0].clone();
    let group_id: BBox<i32, AnyPolicy> = from_value(group_id_val).unwrap();
    // Insert into user table
    let _ = (*bg).prep_exec(
        "INSERT INTO user (user_name, email, privilege) VALUES (?, ?, ?)",
        (stud_name.clone(), stud_email.clone(), "0"),
        context.clone()
    );
    // Find the student_id
    let new_res: Vec<Vec<BBox<Value, AnyPolicy>>> = (*bg).prep_exec("SELECT LAST_INSERT_ID()", 
        (),
        context.clone());
    let student_id_val: BBox<Value, AnyPolicy> = new_res[0][0].clone();
    let student_id: BBox<i32, AnyPolicy> = from_value(student_id_val).unwrap();
    let _ = bg.prep_exec(
        "INSERT INTO enroll (student_id, class_id, group_id) VALUES (?, ?, ?)",
        (student_id, stud_class.clone(), group_id),
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
    stud_class: BBox<String, StudentPolicy>,
    stud_email: BBox<String, StudentPolicy>,
    backend: &State<Arc<Mutex<MySqlBackend>>>,
    context: Context<ContextDataType>,
) -> Json<SuccessResponse> {
    // BUGGY: Make insert query to add this new student into users
    // Hard-coded to always insert to class 2!! This should fail student policy
    let mut bg = backend.lock().unwrap();
    // Make insert query to add this new student into users
    (*bg).prep_exec(
        "INSERT IGNORE INTO `group` (group_name, class_id) VALUES (?, ?) 
        ON DUPLICATE KEY UPDATE group_id = LAST_INSERT_ID(group_id)", 
        (stud_group.clone(), stud_class.clone()), 
        context.clone()
    );
    // Find the group_id
    let new_res: Vec<Vec<BBox<Value, AnyPolicy>>> = (*bg).prep_exec(
        "SELECT LAST_INSERT_ID()", 
        (),
        context.clone());
    let group_id_val: BBox<Value, AnyPolicy> = new_res[0][0].clone();
    let group_id: BBox<i32, AnyPolicy> = from_value(group_id_val).unwrap();
    // Insert into user table
    let _ = (*bg).prep_exec(
        "INSERT INTO user (user_name, email, privilege) VALUES (?, ?, ?)",
        (stud_name.clone(), stud_email.clone(), "0"),
        context.clone()
    );
    // Find the student_id
    let new_res: Vec<Vec<BBox<Value, AnyPolicy>>> = (*bg).prep_exec("SELECT LAST_INSERT_ID()", 
        (),
        context.clone());
    let student_id_val: BBox<Value, AnyPolicy> = new_res[0][0].clone();
    let student_id: BBox<i32, AnyPolicy> = from_value(student_id_val).unwrap();
    let _ = bg.prep_exec(
        "INSERT INTO enroll (student_id, class_id, group_id) VALUES (?, ?, ?)",
        (student_id, 2i32, group_id),
        context.clone()
    );
    drop(bg);

    return Json(SuccessResponse {
        success: true,
        message: "".to_string(),
    });
}

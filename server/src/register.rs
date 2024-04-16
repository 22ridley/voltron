extern crate mysql;
extern crate serde;
use crate::backend::MySqlBackend;
use crate::common::SuccessResponse;
use crate::context::ContextDataType;
use crate::policies::InstructorPolicy;
use alohomora::context::Context;
use alohomora::fold::fold;
use alohomora::pure::{execute_pure, PrivacyPureRegion};
use alohomora::rocket::{post, ContextResponse};
use alohomora::{
    bbox::BBox,
    policy::{AnyPolicy, NoPolicy},
};
use mysql::Value;
use rocket::serde::json::Json;
use rocket::State;
use rocket_firebase_auth::FirebaseToken;
use std::fs::File;
use std::path::Path;
use std::{sync::Arc, sync::Mutex};

#[post("/register_instructor?<instr_name>&<instr_class>&<instr_email>")]
pub fn register_instructor(
    _token: BBox<FirebaseToken, NoPolicy>,
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
    _token: BBox<FirebaseToken, NoPolicy>,
    stud_group: BBox<i32, NoPolicy>,
    stud_name: BBox<String, NoPolicy>,
    stud_class: BBox<i32, NoPolicy>,
    stud_email: BBox<String, NoPolicy>,
    backend: &State<Arc<Mutex<MySqlBackend>>>,
    context: Context<ContextDataType>,
) -> ContextResponse<Json<SuccessResponse>, AnyPolicy, ContextDataType> {
    // Get the names of everyone currently in the database
    let mut bg = backend.lock().unwrap();
    let all_names: Vec<Vec<BBox<Value, AnyPolicy>>> =
        (*bg).prep_exec("SELECT user_name FROM users", (), context.clone());
    drop(bg);
    let all_names_folded: BBox<Vec<Vec<Value>>, AnyPolicy> = fold(all_names).unwrap();

    let response: BBox<Json<SuccessResponse>, AnyPolicy> = execute_pure(
        (stud_group, stud_name, stud_class, stud_email, all_names_folded),
        PrivacyPureRegion::new(|(s_group, s_name, s_class, s_email, names): (i32, String, i32, String, Vec<Vec<Value>>)| {
            // Create variables for failure
            let mut fail: bool = false;
            let mut fail_message: String = "".to_string();

            // Check that this name is not already in the database
            for row in names {
                let name: String = mysql::from_value::<String>(row[0].clone());
                if name == s_name {
                    fail = true;
                    fail_message = format!("Username '{}' was already found in the database", name);
                }
            }
            // Return if either failure occured
            if fail {
                return Json(SuccessResponse {
                    success: false,
                    message: fail_message
                });
            }

            let users_row: Vec<String> = vec![s_name, s_email, "0".to_string(), format!("{}", s_class), format!("{}", s_group)];
            // If this group ID is new, create a new file
            let file_string: String = format!("../group_code/class{}_group{}_code.txt", s_class, s_group);
            let file_name: &Path = Path::new(&file_string);
            if !file_name.is_file() {
                // Open a new file
                let _ = File::create(file_name);
            }

            let mut bg = backend.lock().unwrap();
            // Make insert query to add this new student into users
            let q: &str = "INSERT INTO users (user_name, email, privilege, class_id, group_id) VALUES (?, ?, ?, ?, ?)";
            let _res: Vec<Vec<BBox<Value, AnyPolicy>>> = (*bg).prep_exec(q, users_row, context.clone());
            drop(bg);
            Json(SuccessResponse {
                success: true,
                message: "".to_string()
            })
        })
    ).unwrap();
    ContextResponse::from((response, context))
}

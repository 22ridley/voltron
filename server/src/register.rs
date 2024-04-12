extern crate serde;
extern crate mysql;
use std::path::Path;
use std::{sync::Arc, sync::Mutex};
use alohomora::fold::fold;
use alohomora::pure::{execute_pure, PrivacyPureRegion};
use mysql::Value;
use rocket::serde::json::Json;
use rocket::State;
use crate::backend::MySqlBackend;
use crate::common::SuccessResponse;
use std::fs::File;
use rocket_firebase_auth::FirebaseToken;
use alohomora::{bbox::BBox, policy::{AnyPolicy, NoPolicy}};
use alohomora::rocket::{post, ContextResponse};
use alohomora::context::Context;
use crate::context::ContextDataType;

#[post("/register_instructor?<instr_name>&<instr_class>&<instr_email>")]
pub fn register_instructor(_token: BBox<FirebaseToken, NoPolicy>, 
    instr_name: BBox<String, NoPolicy>,
    instr_class: BBox<String, NoPolicy>, 
    instr_email: BBox<String, NoPolicy>,
    backend: &State<Arc<Mutex<MySqlBackend>>>, 
    context: Context<ContextDataType>
) -> ContextResponse<Json<SuccessResponse>, AnyPolicy, ContextDataType> {
    // Get the names of everyone currently in the database 
    let mut bg = backend.lock().unwrap();
    let all_names: Vec<Vec<BBox<Value, AnyPolicy>>> = (*bg).prep_exec("SELECT user_name FROM users", (), context.clone());
    drop(bg);
    let all_names_folded: BBox<Vec<Vec<Value>>, AnyPolicy> = fold(all_names).unwrap();

    let response: BBox<Json<SuccessResponse>, AnyPolicy> = execute_pure(
        (instr_name, instr_class, instr_email, all_names_folded), 
        PrivacyPureRegion::new(|(i_name, i_class, i_email, names): (String, String, String, Vec<Vec<Value>>)| {
            // Create variables for failure
            let mut fail: bool = false;
            let mut fail_message: String = "".to_string();

            // Check that this name is not already in the database
            for row in names {
                let name: String = mysql::from_value::<String>(row[0].clone());
                if name == *i_name {
                    fail = true;
                    fail_message = format!("Username '{}' was already found in the database", name);
                }
            }

            // Check if student_group is numeric
            if !fail {
                match i_class.parse::<i32>() {
                    Ok(_i32) => (),
                    Err(_e) => {
                        fail = true;
                        fail_message = format!("Class ID '{}' could not be parsed into an integer", i_class);
                    }
                }
            }
            // Return if either failure occured
            if fail {
                return Json(SuccessResponse {
                    success: false,
                    message: fail_message
                });
            } 
            // Assemble values to insert
            let users_row: Vec<&str> = vec![&i_name, &i_email, "1", &i_class, "-1"];

            // Make insert query to add this new instructor
            let mut bg = backend.lock().unwrap();
            let q = "INSERT INTO users (user_name, email, privilege, class_id, group_id) VALUES (?, ?, ?, ?, ?)";

            // send insert query to db
            (*bg).prep_exec(q, users_row, context.clone());
            drop(bg);

            Json(SuccessResponse {
                success: true,
                message: "".to_string()
            })
        })
    ).unwrap();
    ContextResponse::from((response, context))
}

#[post("/register_student?<stud_group>&<stud_name>&<stud_class>&<stud_email>")]
pub fn register_student(_token: BBox<FirebaseToken, NoPolicy>, 
    stud_group: BBox<String, NoPolicy>, 
    stud_name: BBox<String, NoPolicy>,
    stud_class: BBox<String, NoPolicy>, 
    stud_email: BBox<String, NoPolicy>, 
    backend: &State<Arc<Mutex<MySqlBackend>>>,
    context: Context<ContextDataType>) 
    -> ContextResponse<Json<SuccessResponse>, AnyPolicy, ContextDataType> {
    // Get the names of everyone currently in the database 
    let mut bg = backend.lock().unwrap();
    let all_names: Vec<Vec<BBox<Value, AnyPolicy>>> = (*bg).prep_exec("SELECT user_name FROM users", (), context.clone());
    drop(bg);
    let all_names_folded: BBox<Vec<Vec<Value>>, AnyPolicy> = fold(all_names).unwrap();

    let response: BBox<Json<SuccessResponse>, AnyPolicy> = execute_pure(
        (stud_group, stud_name, stud_class, stud_email, all_names_folded), 
        PrivacyPureRegion::new(|(s_group, s_name, s_class, s_email, names): (String, String, String, String, Vec<Vec<Value>>)| {
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
            // Check if student_group is numeric
            if !fail {
                match s_group.parse::<i32>() {
                    Ok(_i32) => (),
                    Err(_e) => {
                        fail = true;
                        fail_message = format!("Group ID '{}' could not be parsed into an integer", s_group);
                    }
                }
            }
            // Return if either failure occured
            if fail {
                return Json(SuccessResponse {
                    success: false,
                    message: fail_message
                });
            }

            let users_row: Vec<&str> = vec![&s_name, &s_email, "0", &s_class, &s_group];
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
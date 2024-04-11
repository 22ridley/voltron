extern crate serde;
extern crate mysql;
use std::path::Path;
use std::{sync::Arc, sync::Mutex};
use mysql::Row;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use crate::backend::MySqlBackend;
use crate::common::{ApiResponse, SuccessResponse};
use std::fs::File;
use rocket_firebase_auth::FirebaseToken;
use alohomora::{bbox::BBox, policy::NoPolicy};
use alohomora::rocket::post;
use alohomora::context::Context;
use crate::policies::ContextDataType;

#[post("/register_instructor?<instr_name>&<instr_class>&<instr_email>")]
pub fn register_instructor(_token: BBox<FirebaseToken, NoPolicy>, 
    instr_name: BBox<String, NoPolicy>,
    instr_class: BBox<String, NoPolicy>, 
    instr_email: BBox<String, NoPolicy>,
    backend: &State<Arc<Mutex<MySqlBackend>>>, 
    context: Context<ContextDataType>
) -> ApiResponse<SuccessResponse> {
    let instructor_name: &str = instr_name.unwrap();
    let class_id: &str = instr_class.unwrap();
    let instructor_email: &str = instr_email.unwrap();
    // Create variables for failure
    let mut fail: bool = false;
    let mut fail_message: String = "".to_string();
    // Get the names of everyone currently in the database 
    let mut bg = backend.lock().unwrap();
    let all_names: Vec<Row> = (*bg).prep_exec("SELECT user_name FROM users", ()).unwrap();
    // Check that this name is not already in the database
    for row in all_names {
        let name: String = row.get(0).unwrap();
        if name == *instructor_name {
            fail = true;
            fail_message = format!("Username '{}' was already found in the database", name);
        }
    }
    // Check if student_group is numeric
    if !fail {
        match class_id.parse::<i32>() {
            Ok(_i32) => (),
            Err(_e) => {
                fail = true;
                fail_message = format!("Class ID '{}' could not be parsed into an integer", class_id);
            }
        }
    }
    // Return if either failure occured
    if fail {
        drop(bg);
        return ApiResponse {
            json: Some(Json(SuccessResponse {
                success: false,
                message: fail_message
            })),
            status: Status::Ok,
        }
    } 
    // Assemble values to insert
    let users_row: Vec<&str> = vec![instructor_name, instructor_email, "1", class_id, "-1"];

    // Make insert query to add this new instructor
    let q = "INSERT INTO users (user_name, email, privilege, class_id, group_id) VALUES (?, ?, ?, ?, ?)";

    // send insert query to db
    let _res: Vec<Row> = (*bg).prep_exec(q, users_row, context).unwrap();
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
pub fn register_student(_token: BBox<FirebaseToken, NoPolicy>, 
    stud_group: BBox<String, NoPolicy>, 
    stud_name: BBox<String, NoPolicy>,
    stud_class: BBox<String, NoPolicy>, 
    stud_email: BBox<String, NoPolicy>, 
    backend: &State<Arc<Mutex<MySqlBackend>>>,
    context: Context<ContextDataType>)-> ApiResponse<SuccessResponse> {
    let mut bg = backend.lock().unwrap();
    let student_group: &str = stud_group.unwrap();
    let student_name: &str = stud_name.unwrap();
    let student_class: &str = stud_class.unwrap();
    let student_email: &str = stud_email.unwrap();
    // Create variables for failure
    let mut fail: bool = false;
    let mut fail_message: String = "".to_string();
    // Get the names of everyone currently in the database 
    let all_names: Vec<Row> = (*bg).prep_exec("SELECT user_name FROM users", (), context).unwrap();
    // Check that this name is not already in the database
    for row in all_names {
        let name: String = row.get(0).unwrap();
        if name == *student_name {
            fail = true;
            fail_message = format!("Username '{}' was already found in the database", name);
        }
    }
    // Check if student_group is numeric
    if !fail {
        match student_group.parse::<i32>() {
            Ok(_i32) => (),
            Err(_e) => {
                fail = true;
                fail_message = format!("Group ID '{}' could not be parsed into an integer", student_group);
            }
        }
    }
    // Return if either failure occured
    if fail {
        drop(bg);
        return ApiResponse {
            json: Some(Json(SuccessResponse {
                success: false,
                message: fail_message
            })),
            status: Status::Ok,
        }
    }

    let users_row: Vec<&str> = vec![student_name, student_email, "0", student_class, student_group];
    // If this group ID is new, create a new file
    let file_string: String = format!("../group_code/class{}_group{}_code.txt", student_class, student_group);
    let file_name: &Path = Path::new(&file_string);
    if !file_name.is_file() {
        // Open a new file
        let _ = File::create(file_name);
    }   

    // Make insert query to add this new student into users
    let q: &str = "INSERT INTO users (user_name, email, privilege, class_id, group_id) VALUES (?, ?, ?, ?, ?)";
    let _res: Vec<Row> = (*bg).prep_exec(q, users_row).unwrap();
    drop(bg);
    ApiResponse {
        json: Some(Json(SuccessResponse {
            success: true,
            message: "".to_string()
        })),
        status: Status::Ok,
    }
}
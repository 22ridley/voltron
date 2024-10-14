use std::{io::Write, path::Path, sync::Arc, sync::Mutex};
use crate::common::{ApiResponse, SuccessResponse};
use mysql::Row;
use rocket::{http::Status, serde::json::Json, Route, State};
use serde::Serialize;
use std::fs::{self, File};
use rocket_firebase_auth::FirebaseToken;
use crate::backend::MySQLBackend;

pub fn routes() -> Vec<Route> {
    routes![student, update]
}

#[derive(Debug, Serialize)]
pub struct StudentResponse {
    pub success: bool,
    pub class_id: i32,
    pub group_id: i32,
    pub contents: String,
}

#[get("/student")]
pub fn student(token: FirebaseToken, backend: &State<Arc<Mutex<MySQLBackend>>>) 
-> ApiResponse<StudentResponse> {
    // Find this student
    let email: String = token.email.unwrap();
    let mut bg: std::sync::MutexGuard<'_, MySQLBackend> = backend.lock().unwrap();
    let user_res: Vec<Row> = (*bg).prep_exec("SELECT * FROM user_enroll WHERE email = ?", vec![email.clone()]).unwrap();
    drop(bg);

    let row: Row = user_res.get(0).unwrap().clone();
    let class_id: i32 = row.get(5).unwrap();
    let group_id: i32 = row.get(6).unwrap();

    // File path to read and write from
    let filepath: String = format!("../group_code/class{}_group{}_code.txt", class_id, group_id);
    
    // Convert group_id to number
    let content_result = fs::read_to_string(filepath);
    let contents: String = match content_result {
        // If this file does not exist, return empty string
        Err(_) => "".to_string(),
        // Otherwise, return the file content
        Ok(msg) => msg.to_string(),
    };

    // Return response
    ApiResponse {
        json: Some(Json(StudentResponse {
            success: true,
            class_id,
            group_id,
            contents,
        })),
        status: Status::Ok,
    }
}

#[post("/update?<text>")]
pub fn update(token: FirebaseToken, backend: &State<Arc<Mutex<MySQLBackend>>>,
    text: Option<&str>) -> ApiResponse<SuccessResponse> {
    // Find this student
    let email: String = token.email.unwrap();
    let mut bg: std::sync::MutexGuard<'_, MySQLBackend> = backend.lock().unwrap();
    let user_res: Vec<Row> = (*bg).prep_exec("SELECT * FROM user_enroll WHERE email = ?", vec![email.clone()]).unwrap();
    drop(bg);

    let row: Row = user_res.get(0).unwrap().clone();
    let class_id: i32 = row.get(5).unwrap();
    let group_id: i32 = row.get(6).unwrap();

    // Open a file in write-only mode, returns `io::Result<File>`
    let filepath: String = format!("../group_code/class{}_group{}_code.txt", class_id, group_id);
    let path: &Path = Path::new(&filepath);
    let mut file: File = File::create(&path).unwrap();

    // Write the new text to the file
    let _bytes_written: Result<usize, std::io::Error> = file.write(text.unwrap().as_bytes());
    // Return response
    ApiResponse {
        json: Some(Json(SuccessResponse {
            success: true,
            message: "".to_string()
        })),
        status: Status::Ok,
    }
}


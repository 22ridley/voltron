use std::{io::Write, path::Path, sync::Arc, sync::Mutex};
use crate::common::{UpdateRequest, AnyResponse, ApiResponse};
use mysql::Row;
use rocket::{form::Form, http::Status, response::Redirect, serde::json::Json, Route, State};
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
pub fn student(token: FirebaseToken, backend: &State<Arc<Mutex<MySQLBackend>>>) -> ApiResponse<StudentResponse> {
    // Find this student
    let email_opt: Option<String> = token.email;
    let mut email: String = "".to_string();
    if email_opt.is_some() {
        email = email_opt.unwrap();
    }
    let mut bg: std::sync::MutexGuard<'_, MySQLBackend> = backend.lock().unwrap();
    let user_res: Vec<Row> = (*bg).prep_exec("SELECT * FROM users WHERE email = ?", vec![email.clone()]).unwrap();
    drop(bg);
    // If the student is not found, return error
    if user_res.len() == 0 {
        return ApiResponse {
            json: Some(Json(StudentResponse {
                success: false,
                class_id: -1,
                group_id: -1,
                contents: "".to_string(),
            })),
            status: Status::InternalServerError,
        }
    }
    let row: Row = user_res.get(0).unwrap().clone();
    let class_id: i32 = row.get(3).unwrap();
    let group_id: i32 = row.get(4).unwrap();

    // File path to read and write from
    let filepath: String = format!("group_code/class{}_group{}_code.txt", class_id, group_id);
    
    // Convert group_id to number
    let contents: String = fs::read_to_string(filepath).expect("Unable to read file");

    // Return response
    ApiResponse {
        json: Some(Json(StudentResponse {
            success: true,
            class_id: class_id,
            group_id: group_id,
            contents: contents,
        })),
        status: Status::Ok,
    }
}

#[post("/", data="<data>")]
pub fn update(data: Form<UpdateRequest>) -> AnyResponse {
    // Open a file in write-only mode, returns `io::Result<File>`
    let filepath: String = format!("group_code/class{}_group{}_code.txt", data.class_id, data.group_id);
    let path: &Path = Path::new(&filepath);
    let mut file: File = File::create(&path).unwrap();

    // Write the new text to the file
    let _bytes_written: Result<usize, std::io::Error> = file.write(data.text.as_bytes());
    return AnyResponse::Redirect(Redirect::to(format!("/student?name={}&class_id={}&group_id={}", data.name, data.class_id, data.group_id)));
}


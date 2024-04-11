use std::{io::Write, path::Path, sync::Arc, sync::Mutex};
use crate::common::SuccessResponse;
use mysql::Value;
use rocket::{serde::json::Json, State};
use serde::Serialize;
use std::fs::{self, File};
use rocket_firebase_auth::FirebaseToken;
use crate::backend::MySqlBackend;
use alohomora::{bbox::BBox, db::from_value, policy::{AnyPolicy, NoPolicy}};
use alohomora::context::Context;
use crate::policies::ContextDataType;
use alohomora::rocket::{get, post, ContextResponse};
use alohomora::pure::{execute_pure, PrivacyPureRegion};

#[derive(Debug, Serialize)]
pub struct StudentResponse {
    pub success: bool,
    pub class_id: i32,
    pub group_id: i32,
    pub contents: String,
}

#[get("/student")]
pub(crate) fn student(token: BBox<FirebaseToken, NoPolicy>, 
    backend: &State<Arc<Mutex<MySqlBackend>>>, 
    context: Context<ContextDataType>) 
-> ContextResponse<Json<StudentResponse>, AnyPolicy, ContextDataType> {
    // Find this student
    let email_bbox: BBox<String, AnyPolicy> = execute_pure(token, 
        PrivacyPureRegion::new(|token: FirebaseToken| {
            // Need the following separate lines to give email a type
            let email: String = token.email.unwrap();
            email
        })
    ).unwrap();
    // let email: String = token.email.unwrap();
    let mut bg: std::sync::MutexGuard<'_, MySqlBackend> = backend.lock().unwrap();
    let user_res: Vec<Vec<BBox<Value, AnyPolicy>>> = (*bg).prep_exec("SELECT * FROM users WHERE email = ?", vec![email_bbox.clone()], context);
    drop(bg);
    // If the student is not found, return error
    if user_res.len() == 0 {
        let response = Json(StudentResponse {
            success: false,
            class_id: -1,
            group_id: -1,
            contents: "".to_string(),
        });
        let response_bbox = BBox::new(response, AnyPolicy::new(NoPolicy{}));
        return ContextResponse::from((response_bbox, context))
    }
    let row: Vec<BBox<Value, AnyPolicy>> = user_res[0];
    let class_id_bbox: BBox<i32, AnyPolicy> = from_value(row[3]).unwrap();
    let group_id_bbox: BBox<i32, AnyPolicy> = from_value(row[4]).unwrap();

    let response = execute_pure((class_id_bbox, group_id_bbox), 
        PrivacyPureRegion::new(|(class_id, group_id): (i32, i32)| {
            // File path to read and write from
            let filepath: String = format!("../group_code/class{}_group{}_code.txt", class_id, group_id);
            
            // Convert group_id to number
            let contents: String = fs::read_to_string(filepath).expect("Unable to read file");

            // Return response
            Json(StudentResponse {
                success: true,
                class_id: class_id,
                group_id: group_id,
                contents: contents,
            })
        })
    ).unwrap();
    ContextResponse::from((response, context))
}

#[post("/update?<text>")]
pub fn update(token: BBox<FirebaseToken, NoPolicy>, 
    text: BBox<String, NoPolicy>,
    backend: &State<Arc<Mutex<MySqlBackend>>>,
    context: Context<ContextDataType>) 
    -> ContextResponse<Json<SuccessResponse>, AnyPolicy, ContextDataType> {
    // Find this student
    let email_bbox: BBox<String, AnyPolicy> = execute_pure(token, 
        PrivacyPureRegion::new(|token: FirebaseToken| {
            // Need the following separate lines to give email a type
            let email: String = token.email.unwrap();
            email
        })
    ).unwrap();
    let mut bg: std::sync::MutexGuard<'_, MySqlBackend> = backend.lock().unwrap();
    let user_res: Vec<Vec<BBox<Value, AnyPolicy>>> = (*bg).prep_exec("SELECT * FROM users WHERE email = ?", vec![email_bbox.clone()], context);
    drop(bg);
    // If the student is not found, return error
    if user_res.len() == 0 {
        let response = Json(SuccessResponse {
            success: false,
            message: "Student not found".to_string()
        });
        let response_bbox = BBox::new(response, AnyPolicy::new(NoPolicy{}));
        return ContextResponse::from((response_bbox, context))
    }
    let row: Vec<BBox<Value, AnyPolicy>> = user_res[0];
    let class_id_bbox: BBox<i32, AnyPolicy> = from_value(row[3]).unwrap();
    let group_id_bbox: BBox<i32, AnyPolicy> = from_value(row[4]).unwrap();

    let response = execute_pure((class_id_bbox, group_id_bbox, text), 
        PrivacyPureRegion::new(|(class_id, group_id, text_u): (i32, i32, String)| {
            // Open a file in write-only mode, returns `io::Result<File>`
            let filepath: String = format!("../group_code/class{}_group{}_code.txt", class_id, group_id);
            let path: &Path = Path::new(&filepath);
            let mut file: File = File::create(&path).unwrap();

            // Write the new text to the file
            let _bytes_written: Result<usize, std::io::Error> = file.write(text_u.as_bytes());
            // Return response
            Json(SuccessResponse {
                success: true,
                message: "".to_string()
            })
        })
    ).unwrap();
    ContextResponse::from((response, context))
}


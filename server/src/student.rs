use std::{collections::HashMap, io::Write, path::Path, sync::{Arc, Mutex}};
use crate::{common::SuccessResponse, policies::VoltronBufferPolicy};
use alohomora::rocket::ResponseBBoxJson;
use mysql::Value;
use rocket::{serde::json::Json, State};
use std::fs::{self, File};
use rocket_firebase_auth::FirebaseToken;
use crate::backend::MySqlBackend;
use alohomora::{bbox::BBox, db::from_value, policy::{AnyPolicy, NoPolicy}, rocket::{JsonResponse, OutputBBoxValue}};
use alohomora::context::Context;
use crate::context::ContextDataType;
use alohomora::rocket::{get, post, ContextResponse};
use alohomora::pure::{execute_pure, PrivacyPureRegion};

pub struct StudentResponse {
    pub success: BBox<bool, AnyPolicy>,
    pub class_id: BBox<i64, AnyPolicy>,
    pub group_id: BBox<i64, AnyPolicy>,
    pub contents: BBox<String, VoltronBufferPolicy>,
}

impl ResponseBBoxJson for StudentResponse {
    fn to_json(self) -> OutputBBoxValue {
        OutputBBoxValue::Object(HashMap::from([
            (String::from("success"), self.success.to_json()),
            (String::from("class_id"), self.class_id.to_json()),
            (String::from("group_id"), self.group_id.to_json()),
            (String::from("contents"), self.contents.to_json()),
        ]))
    }
}

#[get("/student")]
pub(crate) fn student(token: BBox<FirebaseToken, NoPolicy>, 
    backend: &State<Arc<Mutex<MySqlBackend>>>, 
    context: Context<ContextDataType>) 
-> JsonResponse<StudentResponse, ContextDataType> {
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
    let user_res: Vec<Vec<BBox<Value, AnyPolicy>>> = (*bg).prep_exec("SELECT * FROM users WHERE email = ?", vec![email_bbox.clone()], context.clone());
    drop(bg);
    // If the student is not found, return error
    if user_res.len() == 0 {
        let response = StudentResponse {
            success: BBox::new(false, AnyPolicy::new(NoPolicy{})),
            class_id: BBox::new(-1, AnyPolicy::new(NoPolicy{})),
            group_id: BBox::new(-1, AnyPolicy::new(NoPolicy{})),
            contents: BBox::new("".to_string(), VoltronBufferPolicy::new(Some(-1), Some(-1))),
        };
        return JsonResponse::from((response, context))
    }
    let row: Vec<BBox<Value, AnyPolicy>> = user_res[0].clone();
    let class_id_bbox: BBox<i64, AnyPolicy> = from_value(row[3].clone()).unwrap();
    let group_id_bbox: BBox<i64, AnyPolicy> = from_value(row[4].clone()).unwrap();

    let contents = execute_pure((class_id_bbox.clone(), group_id_bbox.clone()), 
        PrivacyPureRegion::new(|(class_id, group_id): (i64, i64)| {
            // File path to read and write from
            let filepath: String = format!("../group_code/class{}_group{}_code.txt", class_id, group_id);
            
            // Convert group_id to number
            let contents: String = fs::read_to_string(filepath).expect("Unable to read file");
            // let contents_bbox: BBox<String, AnyPolicy> = BBox::new(contents, AnyPolicy::new(VoltronBufferPolicy::new(Some(class_id), Some(group_id))));
            contents
        })
    ).unwrap()
    .specialize_policy::<VoltronBufferPolicy>()
    .unwrap();
    let response : StudentResponse = StudentResponse {
        success: BBox::new(true, AnyPolicy::new(NoPolicy{})),
        class_id: class_id_bbox,
        group_id: group_id_bbox,
        contents: contents,
    };
    JsonResponse::from((response, context.clone()))
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
    let user_res: Vec<Vec<BBox<Value, AnyPolicy>>> = (*bg).prep_exec("SELECT * FROM users WHERE email = ?", vec![email_bbox.clone()], context.clone());
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
    let row: Vec<BBox<Value, AnyPolicy>> = user_res[0].clone();
    let class_id_bbox: BBox<i32, AnyPolicy> = from_value(row[3].clone()).unwrap();
    let group_id_bbox: BBox<i32, AnyPolicy> = from_value(row[4].clone()).unwrap();

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
    ContextResponse::from((response, context.clone()))
}


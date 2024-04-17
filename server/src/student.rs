use crate::backend::MySqlBackend;
use crate::common::{read_buffer, write_buffer};
use crate::context::ContextDataType;
use crate::policies::WriteBufferPolicy;
use crate::{common::SuccessResponse, policies::ReadBufferPolicy};
use alohomora::context::Context;
use alohomora::pure::{execute_pure, PrivacyPureRegion};
use alohomora::rocket::ResponseBBoxJson;
use alohomora::rocket::{get, post};
use alohomora::{
    bbox::BBox,
    db::from_value,
    policy::{AnyPolicy, NoPolicy},
    rocket::JsonResponse,
};
use mysql::Value;
use rocket::{serde::json::Json, State};
use rocket_firebase_auth::FirebaseToken;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(ResponseBBoxJson)]
pub struct StudentResponse {
    pub success: BBox<bool, NoPolicy>,
    pub class_id: BBox<i64, ReadBufferPolicy>,
    pub group_id: BBox<i64, ReadBufferPolicy>,
    pub contents: Option<BBox<String, ReadBufferPolicy>>,
}

#[get("/student")]
pub(crate) fn student(
    token: BBox<FirebaseToken, NoPolicy>,
    backend: &State<Arc<Mutex<MySqlBackend>>>,
    context: Context<ContextDataType>,
) -> JsonResponse<StudentResponse, ContextDataType> {
    // Find this student
    let email_bbox: BBox<String, AnyPolicy> = execute_pure(
        token,
        PrivacyPureRegion::new(|token: FirebaseToken| {
            // Need the following separate lines to give email a type
            let email: String = token.email.unwrap();
            email
        }),
    )
    .unwrap();

    let mut bg: std::sync::MutexGuard<'_, MySqlBackend> = backend.lock().unwrap();
    let user_res: Vec<Vec<BBox<Value, AnyPolicy>>> = (*bg).prep_exec(
        "SELECT * FROM users WHERE email = ?",
        vec![email_bbox.clone()],
        context.clone(),
    );
    drop(bg);

    let row: Vec<BBox<Value, AnyPolicy>> = user_res[0].clone();
    let class_id_bbox: BBox<i32, ReadBufferPolicy> = from_value(row[3].clone()).unwrap();
    let group_id_bbox: BBox<i32, ReadBufferPolicy> = from_value(row[4].clone()).unwrap();
    let contents = read_buffer(
        class_id_bbox.clone(),
        group_id_bbox.clone(),
        context.clone(),
    );
    let response: StudentResponse = StudentResponse {
        success: BBox::new(true, NoPolicy::new()),
        class_id: class_id_bbox.into_bbox(),
        group_id: group_id_bbox.into_bbox(),
        contents: Some(contents),
    };
    JsonResponse::from((response, context.clone()))
}

#[post("/update?<text>")]
pub fn update(
    token: BBox<FirebaseToken, NoPolicy>,
    text: BBox<String, WriteBufferPolicy>,
    backend: &State<Arc<Mutex<MySqlBackend>>>,
    context: Context<ContextDataType>,
) -> Json<SuccessResponse> {
    // Find this student
    let email_bbox: BBox<String, AnyPolicy> = execute_pure(
        token,
        PrivacyPureRegion::new(|token: FirebaseToken| {
            // Need the following separate lines to give email a type
            let email: String = token.email.unwrap();
            email
        }),
    )
    .unwrap();
    let mut bg: std::sync::MutexGuard<'_, MySqlBackend> = backend.lock().unwrap();
    let user_res: Vec<Vec<BBox<Value, AnyPolicy>>> = (*bg).prep_exec(
        "SELECT * FROM users WHERE email = ?",
        vec![email_bbox.clone()],
        context.clone(),
    );
    drop(bg);
    // If the student is not found, return error
    if user_res.len() == 0 {
        return Json(SuccessResponse {
            success: false,
            message: "Student not found".to_string(),
        });
    }
    let row: Vec<BBox<Value, AnyPolicy>> = user_res[0].clone();
    let class_id_bbox: BBox<i32, ReadBufferPolicy> = from_value(row[3].clone()).unwrap();
    let group_id_bbox: BBox<i32, ReadBufferPolicy> = from_value(row[4].clone()).unwrap();

    // Needs to be privacy critical region
    write_buffer(class_id_bbox, group_id_bbox, context.clone(), text);

    return Json(SuccessResponse {
        success: true,
        message: "".to_string(),
    });
}

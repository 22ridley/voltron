use crate::backend::MySqlBackend;
use crate::context::ContextDataType;
use crate::policies::{AuthStatePolicy, ReadBufferPolicy, WriteBufferPolicy};
use crate::routes::common::{email_from_token, read_buffer, write_buffer, SuccessResponse};
use mysql::Value;
use rocket::serde::json::Json;
use rocket::State;
use rocket_firebase_auth::FirebaseToken;
use sesame::context::Context;
use sesame::pcon::PCon;
use sesame::policy::AnyPolicy;
use sesame_mysql::from_value;
use sesame_rocket::rocket::{get, post, JsonResponse, ResponsePConJson};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(ResponsePConJson)]
pub struct StudentResponse {
    pub success: bool,
    pub class_id: PCon<i64, ReadBufferPolicy>,
    pub group_id: PCon<i64, ReadBufferPolicy>,
    pub contents: Option<PCon<String, ReadBufferPolicy>>,
}

#[get("/student")]
pub(crate) fn student(
    token: PCon<FirebaseToken, AuthStatePolicy>,
    backend: &State<Arc<Mutex<MySqlBackend>>>,
    context: Context<ContextDataType>,
) -> JsonResponse<StudentResponse, ContextDataType> {
    // Find this student
    let email: PCon<String, AuthStatePolicy> = email_from_token(token);

    let mut bg: std::sync::MutexGuard<'_, MySqlBackend> = backend.lock().unwrap();
    let user_res: Vec<Vec<PCon<Value, AnyPolicy>>> = (*bg).prep_exec(
        "SELECT * FROM users WHERE email = ?",
        vec![email.clone()],
        context.clone(),
    );
    drop(bg);

    let mut row: Vec<PCon<Value, AnyPolicy>> = user_res.into_iter().next().unwrap();
    let group_id = row.swap_remove(4);
    let class_id = row.swap_remove(3);
    let class_id: PCon<i32, ReadBufferPolicy> = from_value(class_id).unwrap();
    let group_id: PCon<i32, ReadBufferPolicy> = from_value(group_id).unwrap();
    let contents = read_buffer(class_id.clone(), group_id.clone(), context.clone());
    let response: StudentResponse = StudentResponse {
        success: true,
        class_id: class_id.into_pcon(),
        group_id: group_id.into_pcon(),
        contents: Some(contents),
    };
    JsonResponse::from((response, context.clone()))
}

#[post("/update?<text>")]
pub fn update(
    token: PCon<FirebaseToken, AuthStatePolicy>,
    text: PCon<String, WriteBufferPolicy>,
    backend: &State<Arc<Mutex<MySqlBackend>>>,
    context: Context<ContextDataType>,
) -> Json<SuccessResponse> {
    // Find this student
    let email: PCon<String, AuthStatePolicy> = email_from_token(token);
    let mut bg: std::sync::MutexGuard<'_, MySqlBackend> = backend.lock().unwrap();
    let user_res: Vec<Vec<PCon<Value, AnyPolicy>>> = (*bg).prep_exec(
        "SELECT * FROM users WHERE email = ?",
        vec![email.clone()],
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

    let mut row: Vec<PCon<Value, AnyPolicy>> = user_res.into_iter().next().unwrap();
    let group_id = row.swap_remove(4);
    let class_id = row.swap_remove(3);
    let class_id: PCon<i32, ReadBufferPolicy> = from_value(class_id).unwrap();
    let group_id: PCon<i32, ReadBufferPolicy> = from_value(group_id).unwrap();

    // Needs to be privacy critical region
    write_buffer(class_id, group_id, context.clone(), text);

    Json(SuccessResponse {
        success: true,
        message: "".to_string(),
    })
}

// Buggy version of endpoint!
#[post("/update_buggy?<text>")]
pub fn update_buggy(
    token: PCon<FirebaseToken, AuthStatePolicy>,
    text: PCon<String, WriteBufferPolicy>,
    backend: &State<Arc<Mutex<MySqlBackend>>>,
    context: Context<ContextDataType>,
) -> Json<SuccessResponse> {
    // Find this student
    let email: PCon<String, AuthStatePolicy> = email_from_token(token);
    let mut bg: std::sync::MutexGuard<'_, MySqlBackend> = backend.lock().unwrap();
    let user_res: Vec<Vec<PCon<Value, AnyPolicy>>> = (*bg).prep_exec(
        "SELECT * FROM users WHERE email = ?",
        vec![email.clone()],
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

    let mut row: Vec<PCon<Value, AnyPolicy>> = user_res.into_iter().next().unwrap();
    let class_id = row.swap_remove(3);
    let class_id: PCon<i32, ReadBufferPolicy> = from_value(class_id).unwrap();
    // Cooking up the wrong group_id to write to a different buffer
    let wrong_group_id = PCon::new(2, ReadBufferPolicy::new(0, 2));

    // BUGGY: We try to write to the buffer of the wrong group
    write_buffer(class_id, wrong_group_id, context.clone(), text);

    Json(SuccessResponse {
        success: true,
        message: "".to_string(),
    })
}

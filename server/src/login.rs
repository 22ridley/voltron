use alohomora::db::from_value;
use alohomora::policy::AnyPolicy;
use alohomora::pure::{execute_pure, PrivacyPureRegion};
use mysql::Value;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use rocket_firebase_auth::FirebaseToken;
use serde::Serialize;
use crate::backend::MySqlBackend;
use std::{sync::Arc, sync::Mutex};
use crate::common::ApiResponse;
use alohomora::{bbox::BBox, policy::NoPolicy};
use alohomora::rocket::get;
use alohomora::context::Context;
use crate::policies::ContextDataType;

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub name: String,
    pub email: String,
    pub privilege: i32,
}

#[get("/login")]
pub(crate) fn login(
    token: BBox<FirebaseToken, NoPolicy>, 
    backend: &State<Arc<Mutex<MySqlBackend>>>, 
    context: Context<ContextDataType>
) -> ApiResponse<LoginResponse> {
    let email_bbox: BBox<String, AnyPolicy> = execute_pure(token, 
        PrivacyPureRegion::new(|token: FirebaseToken| {
            // Need the following separate lines to give email a type
            let email: String = token.email.unwrap();
            email
        })
    ).unwrap();

    let mut bg = backend.lock().unwrap();
    let user_res: Vec<Vec<BBox<Value, AnyPolicy>>> = (*bg).prep_exec("SELECT * FROM users WHERE email = ?", vec![email_bbox.clone()], context.clone());
    drop(bg);

    let mut response: BBox<ApiResponse<LoginResponse>, AnyPolicy>;
    if user_res.len() == 0 {
        response = execute_pure(email_bbox, 
            PrivacyPureRegion::new(|email: String| {
                ApiResponse {
                    json: Some(Json(LoginResponse {
                        success: true,
                        name: "".to_string(),
                        privilege: -1,
                        email: email,
                    })),
                    status: Status::Ok,
                }}))
        .unwrap();
    } else {
        let row: Vec<BBox<Value, AnyPolicy>> = user_res.get(0).unwrap().clone();
        let name_bbox: BBox<String, AnyPolicy> = from_value(row[0]).unwrap();
        let priv_bbox: BBox<i32, AnyPolicy> = from_value(row[2]).unwrap();
        response = execute_pure((email_bbox, name_bbox, priv_bbox), 
            PrivacyPureRegion::new(|(email, name, privl): (String, String, i32)| {
                ApiResponse {
                    json: Some(Json(LoginResponse {
                        success: true,
                        name: name,
                        privilege: privl,
                        email: email,
                    })),
                    status: Status::Ok,
                }
            })
        ).unwrap();
    }
    response
}

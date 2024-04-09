use alohomora::policy::AnyPolicy;
use alohomora::pure::{execute_pure, PrivacyPureRegion};
use mysql::{Row, Value};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use rocket_firebase_auth::FirebaseToken;
use serde::Serialize;
use crate::backend::MySQLBackend;
use std::{sync::Arc, sync::Mutex};
use crate::common::ApiResponse;
use alohomora::rocket::{BBoxResponse, BBoxResponseResult};
use alohomora::{bbox::BBox, policy::NoPolicy};
use alohomora_derive::get;

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub name: String,
    pub email: String,
    pub privilege: i32,
}

#[get("/login")]
pub(crate) fn login(
    token: BBox<FirebaseToken, NoPolicy>, backend: &State<Arc<Mutex<MySQLBackend>>>
) -> ApiResponse<LoginResponse> {
    // Statically analyzed region in a closure (look at websubmit)
    // Use a match statement instead
    /*
    let email = match email_opt {
        Some(email) => email
        None(email) => Panic with a custom error
    }
     */
    /*
        Statically checked region is called PPR
        execute_pure(token, PrivacyPureRegion::new(|token| {BBox::new(token.email.unwrap(), NoPolicy)}))
        returns BBox<String>
     */
    let email_bbox = execute_pure(token, 
        PrivacyPureRegion::new(|token: FirebaseToken| {
            // Need two separate lines to give email a type
            let email: String = token.email.unwrap();
            email
        })
    ).unwrap();
    let user_res_bbox = execute_pure(email_bbox, 
        PrivacyPureRegion::new(|email: String| {
            let mut bg: std::sync::MutexGuard<'_, MySQLBackend> = backend.lock().unwrap();
            let user_res: Vec<Vec<BBox<Value, AnyPolicy>>> = (*bg).prep_exec("SELECT * FROM users WHERE email = ?", vec![email.clone()]);
            drop(bg);
            user_res;
        })
    ).unwrap();

    let user_res_bbox: Vec<BBox<Vec<Value>, AnyPolicy>> = user_res_bbox.into();
    let response = execute_pure((email_bbox, user_res_bbox),
        PrivacyPureRegion::new(|(email: String, user_res: Vec<Bbox<Vec<Value>, AnyPolicy>>)| {
            if user_res.len() == 0 {
                return ApiResponse {
                    json: Some(Json(LoginResponse {
                        success: false,
                        name: "".to_string(),
                        email: email,
                        privilege: -1,
                    })),
                    status: Status::InternalServerError,
                }
            }
            let row: Row = user_res.get(0).unwrap().clone();
            let user_name: String = row.get(0).unwrap();
            let privilege: i32 =  row.get(2).unwrap();
            // Return response
            return ApiResponse {
                json: Some(Json(LoginResponse {
                    success: true,
                    name: user_name,
                    privilege: privilege,
                    email: email,
                })),
                status: Status::Ok,
            }
        })
    ).unwrap();
    response;
}

use mysql::Row;
use rocket::state::State;
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
) -> String {
    let email_opt: Option<String> = token.email;
    let mut email: String = "".to_string();
    if email_opt.is_some() {
        email = email_opt.unwrap();
    }
    let mut bg: std::sync::MutexGuard<'_, MySQLBackend> = backend.lock().unwrap();
    let user_res: Vec<Row> = (*bg).prep_exec("SELECT * FROM users WHERE email = ?", vec![email.clone()]).unwrap();
    drop(bg);
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
    ApiResponse {
        json: Some(Json(LoginResponse {
            success: true,
            name: user_name,
            privilege: privilege,
            email: email,
        })),
        status: Status::Ok,
    }
}

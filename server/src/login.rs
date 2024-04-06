use alohomora::pure::{execute_pure, PrivacyPureRegion};
use mysql::Row;
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
) -> String {
    // Statically analyzed region in a closure (look at websubmit)
    // Use a match statement instead
    /*
    let email = match email_opt {
        Some(email) => email
        None(email) => Panic with a custom error
    }
     */
    let email_bbox = execute_pure(token, PrivacyPureRegion::new(|token: FirebaseToken| token.email.unwrap()));
    let email: String = token.email.unwrap();
    /*
    Statically checked region is called PPR
    execute_pure(token, PrivacyPureRegion::new(|token| {BBox::new(token.email.unwrap(), NoPolicy)}))
    returns BBox<String>
     */
    let mut bg: std::sync::MutexGuard<'_, MySQLBackend> = backend.lock().unwrap();
    let user_res: Vec<Row> = (*bg).prep_exec("SELECT * FROM users WHERE email = ?", vec![email.clone()]).unwrap();
    drop(bg);
    if user_res.len() == 0 {
        return "fail".to_string();
        // return ApiResponse {
        //     json: Some(Json(LoginResponse {
        //         success: false,
        //         name: "".to_string(),
        //         email: email,
        //         privilege: -1,
        //     })),
        //     status: Status::InternalServerError,
        // }
    }
    let row: Row = user_res.get(0).unwrap().clone();
    let user_name: String = row.get(0).unwrap();
    let privilege: i32 =  row.get(2).unwrap();
    // Return response
    return "success".to_string();
    // ApiResponse {
    //     json: Some(Json(LoginResponse {
    //         success: true,
    //         name: user_name,
    //         privilege: privilege,
    //         email: email,
    //     })),
    //     status: Status::Ok,
    // }
}

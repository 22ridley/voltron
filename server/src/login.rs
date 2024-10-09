use mysql::Row;
use rocket::{
    get,
    http::Status,
    routes,
    serde::json::Json,
    State,
    Route
};
use rocket_firebase_auth::FirebaseToken;
use serde::Serialize;
use crate::backend::MySQLBackend;
use std::{sync::Arc, sync::Mutex};
use crate::common::ApiResponse;

pub fn routes() -> Vec<Route> {
    routes![login]
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub name: String,
    pub email: String,
    pub privilege: i32,
}

#[get("/login")]
async fn login(
    token: FirebaseToken, backend: &State<Arc<Mutex<MySQLBackend>>>
) -> ApiResponse<LoginResponse> {
    let email: String = token.email.unwrap();
    let mut bg: std::sync::MutexGuard<'_, MySQLBackend> = backend.lock().unwrap();
    let user_res: Vec<Row> = (*bg).prep_exec("SELECT * FROM user WHERE email = ?", vec![email.clone()]).unwrap();
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
    // Iterate over each column in the row
    let user_name: String = row.get(1).unwrap();
    let privilege: i32 =  row.get(3).unwrap();
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

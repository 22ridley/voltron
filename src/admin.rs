use crate::common::{Instructor, ApiResponse};
use crate::backend::MySQLBackend;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{Route, State};
use serde::Serialize;
use std::sync::{Arc, Mutex};
use rocket_firebase_auth::FirebaseToken;

pub fn routes() -> Vec<Route> {
    routes![admin]
}

#[derive(Debug, Serialize)]
pub struct AdminResponse {
    pub success: bool,
    pub instructors: Vec<Instructor>
}

#[get("/admin")]
pub fn admin(_token: FirebaseToken, backend: &State<Arc<Mutex<MySQLBackend>>>) 
    -> ApiResponse<AdminResponse> {
    // Get list of all students
    let mut bg: std::sync::MutexGuard<'_, MySQLBackend> = backend.lock().unwrap();
    let instructors_res: Vec<Instructor> = (*bg).prep_exec("SELECT * FROM users WHERE privilege = 1", ()).unwrap();
    drop(bg);

    // Return response
    ApiResponse {
        json: Some(Json(AdminResponse {
            success: true,
            instructors: instructors_res
        })),
        status: Status::Ok,
    }
}
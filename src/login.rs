use std::io::Cursor;

use mysql::Row;
use rocket::{
    get,
    http::{ContentType, Status},
    post,
    response,
    routes,
    serde::json::Json,
    Request,
    Response,
    Route,
    response::Redirect, State, form::Form
};
use rocket_firebase_auth::FirebaseToken;
use serde::Serialize;
use crate::backend::MySQLBackend;
use std::{sync::Arc, sync::Mutex};

pub fn routes() -> Vec<Route> {
    routes![login]
}

/// The struct we return for success responses (200s)
#[derive(Debug)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    pub json: Option<Json<T>>,
    pub status: Status,
}

/// Implements the `Responder` trait for Rocket, so we can simply return a for
/// endpoint functions, result and Rocket takes care of the rest.
impl<'r, T: Serialize> response::Responder<'r, 'r> for ApiResponse<T> {
    fn respond_to(self, req: &'r Request) -> response::Result<'r> {
        Response::build_from(self.json.respond_to(req)?)
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

/// The struct we return for error responses (400s, 500s)
#[derive(Debug)]
pub struct ApiError {
    pub error: String,
    pub status: Status,
}

/// Implements the `Responder` trait, much like for `ApiResponse`, but for `ApiError`
impl<'a, 'o: 'a> response::Responder<'a, 'o> for ApiError {
    fn respond_to(self, _: &'a Request) -> Result<Response<'o>, Status> {
        Response::build()
            .status(self.status)
            .sized_body(self.error.len(), Cursor::new(self.error))
            .ok()
    }
}

#[derive(Debug, Serialize)]
pub struct VerifyTokenResponse {
    pub uid: String,
}

#[derive(Debug, Serialize)]
pub struct ProtectedEndpointResponse {
    pub success: bool,
    pub name: String,
    pub email: String,
    pub privilege: i32,
}

#[get("/login")]
async fn login(
    token: FirebaseToken, backend: &State<Arc<Mutex<MySQLBackend>>>
) -> ApiResponse<ProtectedEndpointResponse> {
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
            json: Some(Json(ProtectedEndpointResponse {
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
        json: Some(Json(ProtectedEndpointResponse {
            success: true,
            name: user_name,
            privilege: privilege,
            email: email,
        })),
        status: Status::Ok,
    }
}

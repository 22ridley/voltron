use crate::backend::MySqlBackend;
use crate::common::email_bbox_from_token;
use crate::context::ContextDataType;
use alohomora::context::Context;
use alohomora::db::from_value;
use alohomora::policy::AnyPolicy;
use alohomora::rocket::{get, JsonResponse, ResponseBBoxJson};
use alohomora::{bbox::BBox, policy::NoPolicy};
use mysql::Value;
use rocket::State;
use rocket_firebase_auth::FirebaseToken;
use std::collections::HashMap;
use std::{sync::Arc, sync::Mutex};

#[derive(ResponseBBoxJson)]
pub struct LoginResponse {
    pub success: bool,
    pub name: BBox<String, AnyPolicy>,
    pub email: BBox<String, NoPolicy>,
    pub privilege: BBox<i32, AnyPolicy>,
}

#[get("/login")]
pub(crate) fn login(
    token: BBox<FirebaseToken, NoPolicy>,
    backend: &State<Arc<Mutex<MySqlBackend>>>,
    context: Context<ContextDataType>,
) -> JsonResponse<LoginResponse, ContextDataType> {
    let email_bbox: BBox<String, NoPolicy> = email_bbox_from_token(token);

    let mut bg = backend.lock().unwrap();
    let user_res: Vec<Vec<BBox<Value, AnyPolicy>>> = (*bg).prep_exec(
        "SELECT * FROM users WHERE email = ?",
        vec![email_bbox.clone()],
        context.clone(),
    );
    drop(bg);

    let response: LoginResponse;
    if user_res.len() == 0 {
        response = LoginResponse {
            success: false,
            name: BBox::new("".to_string(), AnyPolicy::new(NoPolicy {})),
            privilege: BBox::new(-1, AnyPolicy::new(NoPolicy {})),
            email: email_bbox,
        };
    } else {
        let row: Vec<BBox<Value, AnyPolicy>> = user_res.get(0).unwrap().clone();
        let name_bbox: BBox<String, AnyPolicy> = from_value(row[0].clone()).unwrap();
        let priv_bbox: BBox<i32, AnyPolicy> = from_value(row[2].clone()).unwrap();
        response = LoginResponse {
            success: true,
            name: name_bbox,
            privilege: priv_bbox,
            email: email_bbox,
        };
    }
    JsonResponse::from((response, context.clone()))
}

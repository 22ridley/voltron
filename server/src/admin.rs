use crate::backend::MySqlBackend;
use crate::common::Instructor;
use crate::context::ContextDataType;
use alohomora::context::Context;
use alohomora::db::from_value;
use alohomora::fold::fold;
use alohomora::pure::{execute_pure, PrivacyPureRegion};
use alohomora::rocket::{get, ContextResponse};
use alohomora::{
    bbox::BBox,
    policy::{AnyPolicy, NoPolicy},
};
use mysql::Value;
use rocket::serde::json::Json;
use rocket::State;
use rocket_firebase_auth::FirebaseToken;
use serde::Serialize;
use std::sync::{Arc, Mutex};

#[derive(Debug, Serialize)]
pub struct AdminResponse {
    pub success: bool,
    pub instructors: Vec<Instructor>,
}

#[get("/admin")]
pub(crate) fn admin(
    _token: BBox<FirebaseToken, NoPolicy>,
    backend: &State<Arc<Mutex<MySqlBackend>>>,
    context: Context<ContextDataType>,
) -> ContextResponse<Json<AdminResponse>, AnyPolicy, ContextDataType> {
    // Verify that this user is admin?
    // Get list of all instructors
    let mut bg: std::sync::MutexGuard<'_, MySqlBackend> = backend.lock().unwrap();
    let instructors_bbox: Vec<Vec<BBox<Value, AnyPolicy>>> = (*bg).prep_exec(
        "SELECT * FROM users WHERE privilege = 1",
        (),
        context.clone(),
    );
    drop(bg);

    let mut instr_vec_bbox: Vec<BBox<Instructor, AnyPolicy>> = Vec::new();
    for instr in instructors_bbox.iter() {
        let name_bbox: BBox<String, AnyPolicy> = from_value(instr[0].clone()).unwrap();
        let class_id_bbox: BBox<i32, AnyPolicy> = from_value(instr[3].clone()).unwrap();
        let new_instr: BBox<Instructor, AnyPolicy> = execute_pure(
            (name_bbox, class_id_bbox),
            PrivacyPureRegion::new(|(name, class_id): (String, i32)| Instructor {
                name,
                class_id,
                index: 0,
            }),
        )
        .unwrap();
        instr_vec_bbox.push(new_instr)
    }

    // Fold to move BBox to outside
    let instr_bbox_vec: BBox<Vec<Instructor>, AnyPolicy> = fold(instr_vec_bbox).unwrap();
    // Return response
    let response = execute_pure(
        instr_bbox_vec,
        PrivacyPureRegion::new(|instr_vec| {
            Json(AdminResponse {
                success: true,
                instructors: instr_vec,
            })
        }),
    )
    .unwrap();
    ContextResponse::from((response, context))
}

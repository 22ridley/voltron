use crate::backend::MySqlBackend;
use crate::common::Instructor;
use crate::context::ContextDataType;
use crate::policies::AuthStatePolicy;
use alohomora::context::Context;
use alohomora::db::from_value;
use alohomora::rocket::{get, JsonResponse, ResponseBBoxJson};
use alohomora::{bbox::BBox, policy::AnyPolicy};
use mysql::Value;
use rocket::State;
use rocket_firebase_auth::FirebaseToken;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(ResponseBBoxJson)]
pub struct AdminResponse {
    pub success: bool,
    pub instructors: Vec<Instructor>,
}

#[get("/admin")]
pub(crate) fn admin(
    _token: BBox<FirebaseToken, AuthStatePolicy>,
    backend: &State<Arc<Mutex<MySqlBackend>>>,
    context: Context<ContextDataType>,
) -> JsonResponse<AdminResponse, ContextDataType> {
    // Verify that this user is admin?
    // Get list of all instructors
    let mut bg: std::sync::MutexGuard<'_, MySqlBackend> = backend.lock().unwrap();
    let instructors_bbox: Vec<Vec<BBox<Value, AnyPolicy>>> = (*bg).prep_exec(
        "SELECT * FROM users WHERE privilege = 1",
        (),
        context.clone(),
    );
    drop(bg);

    let mut instr_vec_bbox: Vec<Instructor> = Vec::new();
    for instr in instructors_bbox.iter() {
        let name_bbox: BBox<String, AnyPolicy> = from_value(instr[0].clone()).unwrap();
        let class_id_bbox: BBox<i32, AnyPolicy> = from_value(instr[3].clone()).unwrap();
        let new_instr: Instructor = Instructor {
            name: name_bbox,
            class_id: class_id_bbox,
        };
        instr_vec_bbox.push(new_instr)
    }

    let response = AdminResponse {
        success: true,
        instructors: instr_vec_bbox,
    };
    JsonResponse::from((response, context.clone()))
}

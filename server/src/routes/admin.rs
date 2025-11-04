use crate::backend::MySqlBackend;
use crate::context::ContextDataType;
use crate::policies::AuthStatePolicy;
use crate::routes::common::Instructor;
use sesame::context::Context;
use sesame::pcon::PCon;
use sesame::policy::AnyPolicy;
use sesame_mysql::from_value;
use sesame_rocket::rocket::{get, JsonResponse, ResponsePConJson};

use mysql::Value;
use rocket::State;
use rocket_firebase_auth::FirebaseToken;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(ResponsePConJson)]
pub struct AdminResponse {
    pub success: bool,
    pub instructors: Vec<Instructor>,
}

#[get("/admin")]
pub(crate) fn admin(
    _token: PCon<FirebaseToken, AuthStatePolicy>,
    backend: &State<Arc<Mutex<MySqlBackend>>>,
    context: Context<ContextDataType>,
) -> JsonResponse<AdminResponse, ContextDataType> {
    // Verify that this user is admin?
    // Get list of all instructors
    let mut bg: std::sync::MutexGuard<'_, MySqlBackend> = backend.lock().unwrap();
    let result: Vec<Vec<PCon<Value, AnyPolicy>>> = (*bg).prep_exec(
        "SELECT * FROM users WHERE privilege = 1",
        (),
        context.clone(),
    );
    drop(bg);

    let mut instructors: Vec<Instructor> = Vec::new();
    for mut instr in result.into_iter() {
        let class_id = instr.swap_remove(3);
        let name = instr.swap_remove(0);
        let name: PCon<String, AnyPolicy> = from_value(name).unwrap();
        let class_id: PCon<i32, AnyPolicy> = from_value(class_id).unwrap();
        let new_instr: Instructor = Instructor {
            name: name.into_any_policy_no_clone(),
            class_id,
        };
        instructors.push(new_instr)
    }

    let response = AdminResponse {
        success: true,
        instructors,
    };
    JsonResponse::from((response, context.clone()))
}

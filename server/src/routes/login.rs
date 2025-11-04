use crate::backend::MySqlBackend;
use crate::context::ContextDataType;
use crate::policies::AuthStatePolicy;
use crate::routes::common::email_from_token;
use mysql::Value;
use rocket::State;
use rocket_firebase_auth::FirebaseToken;
use sesame::context::Context;
use sesame::pcon::PCon;
use sesame::policy::{AnyPolicy, NoPolicy};
use sesame_mysql::from_value;
use sesame_rocket::rocket::{get, JsonResponse, ResponsePConJson};
use std::collections::HashMap;
use std::{sync::Arc, sync::Mutex};

#[derive(ResponsePConJson)]
pub struct LoginResponse {
    pub success: bool,
    pub name: PCon<String, AnyPolicy>,
    pub email: PCon<String, AnyPolicy>,
    pub privilege: PCon<i32, AnyPolicy>,
}

#[get("/login")]
pub(crate) fn login(
    token: PCon<FirebaseToken, AuthStatePolicy>,
    backend: &State<Arc<Mutex<MySqlBackend>>>,
    context: Context<ContextDataType>,
) -> JsonResponse<LoginResponse, ContextDataType> {
    let email: PCon<String, AuthStatePolicy> = email_from_token(token);

    let mut bg = backend.lock().unwrap();
    let user_res: Vec<Vec<PCon<Value, AnyPolicy>>> = (*bg).prep_exec(
        "SELECT * FROM users WHERE email = ?",
        vec![email.clone()],
        context.clone(),
    );
    drop(bg);

    let response: LoginResponse;
    if user_res.len() == 0 {
        response = LoginResponse {
            success: false,
            name: PCon::new("".to_string(), AnyPolicy::new(NoPolicy {})),
            privilege: PCon::new(-1, AnyPolicy::new(NoPolicy {})),
            email: PCon::new("".to_string(), AnyPolicy::new(NoPolicy {})),
        };
    } else {
        let mut row: Vec<PCon<Value, AnyPolicy>> = user_res.into_iter().next().unwrap();
        let privilege = row.swap_remove(2);
        let email = row.swap_remove(1);
        let name = row.swap_remove(0);
        let privilege: PCon<i32, AnyPolicy> = from_value(privilege).unwrap();
        let email: PCon<String, AnyPolicy> = from_value(email).unwrap();
        let name: PCon<String, AnyPolicy> = from_value(name).unwrap();

        response = LoginResponse {
            success: true,
            name,
            privilege,
            email,
        };
    }
    JsonResponse::from((response, context.clone()))
}

// Buggy version of endpoint!
#[get("/login_email_buggy")]
pub(crate) fn login_email_buggy(
    _token: PCon<FirebaseToken, AuthStatePolicy>,
    backend: &State<Arc<Mutex<MySqlBackend>>>,
    context: Context<ContextDataType>,
) -> JsonResponse<LoginResponse, ContextDataType> {
    let mut bg = backend.lock().unwrap();
    // BUGGY: We hard-code the email address "22ridleysk@gmail.com" and try to
    // access their data from the database, even though that is not the email
    // address attached to our firebase token
    let user_res: Vec<Vec<PCon<Value, AnyPolicy>>> = (*bg).prep_exec(
        "SELECT * FROM users WHERE email = ?",
        vec!["22ridleysk@gmail.com"],
        context.clone(),
    );
    drop(bg);

    let response: LoginResponse;
    if user_res.len() == 0 {
        response = LoginResponse {
            success: false,
            name: PCon::new("".to_string(), AnyPolicy::new(NoPolicy {})),
            privilege: PCon::new(-1, AnyPolicy::new(NoPolicy {})),
            email: PCon::new("".to_string(), AnyPolicy::new(NoPolicy {})),
        };
    } else {
        let mut row: Vec<PCon<Value, AnyPolicy>> = user_res.into_iter().next().unwrap();
        let privilege = row.swap_remove(2);
        let name = row.swap_remove(0);
        let email = row.swap_remove(1);
        let privilege: PCon<i32, AnyPolicy> = from_value(privilege).unwrap();
        let email: PCon<String, AnyPolicy> = from_value(email).unwrap();
        let name: PCon<String, AnyPolicy> = from_value(name).unwrap();
        response = LoginResponse {
            success: true,
            name,
            privilege,
            email,
        };
    }
    JsonResponse::from((response, context.clone()))
}

// Another buggy version of endpoint!
#[get("/login_auth_buggy")]
pub(crate) fn login_auth_buggy(
    token: PCon<FirebaseToken, AuthStatePolicy>,
    backend: &State<Arc<Mutex<MySqlBackend>>>,
    context: Context<ContextDataType>,
) -> JsonResponse<LoginResponse, ContextDataType> {
    let email: PCon<String, AuthStatePolicy> = email_from_token(token);

    let mut bg = backend.lock().unwrap();
    let user_res: Vec<Vec<PCon<Value, AnyPolicy>>> = (*bg).prep_exec(
        "SELECT * FROM users WHERE email = ?",
        vec![email.clone()],
        context.clone(),
    );
    drop(bg);

    let response: LoginResponse;
    if user_res.len() == 0 {
        response = LoginResponse {
            success: false,
            name: PCon::new("".to_string(), AnyPolicy::new(NoPolicy {})),
            privilege: PCon::new(-1, AnyPolicy::new(NoPolicy {})),
            // BUGGY: We try to return email, which has the AuthState policy, and thus
            // should only be accessed by the database
            email: email.into_any_policy_no_clone(),
        };
    } else {
        let mut row: Vec<PCon<Value, AnyPolicy>> = user_res.into_iter().next().unwrap();
        let privilege = row.swap_remove(2);
        let name = row.swap_remove(0);
        let privilege: PCon<i32, AnyPolicy> = from_value(privilege).unwrap();
        let name: PCon<String, AnyPolicy> = from_value(name).unwrap();
        response = LoginResponse {
            success: true,
            name,
            privilege,
            // BUGGY: We try to return email, which has the AuthState policy, and thus
            // should only be accessed by the database
            email: email.into_any_policy_no_clone(),
        };
    }
    JsonResponse::from((response, context.clone()))
}

use alohomora::context::Context;
use alohomora::db::from_value;
use alohomora::fold::fold;
use alohomora::policy::AnyPolicy;
use alohomora::pure::{execute_pure, PrivacyPureRegion};
use alohomora::AlohomoraType;
use mysql::{Row, Value};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use rocket_firebase_auth::FirebaseToken;
use serde::Serialize;
use crate::backend::MySqlBackend;
use crate::config::Config;
use std::{sync::Arc, sync::Mutex};
use crate::common::ApiResponse;
use alohomora::rocket::{BBoxRequest, BBoxRequestOutcome, BBoxResponse, BBoxResponseResult, FromBBoxRequest};
use alohomora::{bbox::BBox, policy::NoPolicy};
use alohomora_derive::get;
use crate::policies::QueryableOnly;

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub name: String,
    pub email: String,
    pub privilege: i32,
}

// Custom developer defined payload attached to every context.
#[derive(AlohomoraType, Clone)]
#[alohomora_out_type(verbatim = [db, config])]
pub struct ContextDataType {
    pub user: Option<BBox<String, NoPolicy>>,
    pub db: Arc<Mutex<MySqlBackend>>,
    pub config: Config,
}

// Build the custom payload for the context given HTTP request.
#[rocket::async_trait]
impl<'a, 'r> FromBBoxRequest<'a, 'r> for ContextDataType {
    type BBoxError = ();

    async fn from_bbox_request(
        request: BBoxRequest<'a, 'r>,
    ) -> BBoxRequestOutcome<Self, Self::BBoxError> {
        let db: &State<Arc<Mutex<MySqlBackend>>> = request.guard().await.unwrap();
        let config: &State<Config> = request.guard().await.unwrap();

        // Find user using ApiKey token from cookie.
        let apikey = request
            .cookies()
            .get::<QueryableOnly>("apikey");
        let user = match apikey {
            None => None,
            Some(apikey) => {
                let apikey = apikey.value().to_owned();
                let mut bg = db.lock().unwrap();
                let res = bg.prep_exec(
                    "SELECT * FROM users WHERE apikey = ?",
                    (apikey,),
                    Context::empty(),
                );
                drop(bg);
                if res.len() > 0 {
                    Some(from_value(res[0][0].clone()).unwrap())
                }
                else {
                    None
                }
            }
        };

        request
            .route()
            .and_then(|_| {
                Some(ContextDataType {
                    user,
                    db: db.inner().clone(),
                    config: config.inner().clone(),
                })
            })
            .into_outcome((
                Status::InternalServerError,
                (),
            ))
    }
}

#[get("/login")]
pub(crate) fn login(
    token: BBox<FirebaseToken, NoPolicy>, backend: &State<Arc<Mutex<MySqlBackend>>>, 
    context: Context<ContextDataType>
) -> ApiResponse<LoginResponse> {
    // Statically analyzed region in a closure (look at websubmit)
    // Use a match statement instead
    /*
    let email = match email_opt {
        Some(email) => email
        None(email) => Panic with a custom error
    }
     */
    /*
        Statically checked region is called PPR
        execute_pure(token, PrivacyPureRegion::new(|token| {BBox::new(token.email.unwrap(), NoPolicy)}))
        returns BBox<String>
     */
    let email_bbox = execute_pure(token, 
        PrivacyPureRegion::new(|token: FirebaseToken| {
            // Need the following separate lines to give email a type
            let email: String = token.email.unwrap();
            email
        })
    ).unwrap();

    let user_res_bbox = execute_pure(email_bbox, 
        PrivacyPureRegion::new(|email: String| {
            let mut bg: std::sync::MutexGuard<'_, MySqlBackend> = backend.lock().unwrap();
            let user_res: Vec<Vec<BBox<Value, AnyPolicy>>> = (*bg).prep_exec("SELECT * FROM users WHERE email = ?", vec![email.clone()]);
            drop(bg);
            user_res
        })
    ).unwrap();

    // let user_res_bbox: Vec<BBox<Vec<Value>, AnyPolicy>> = fold(user_res_bbox);
    let response = execute_pure((email_bbox, user_res_bbox),
        PrivacyPureRegion::new(|(email, user_res)| {
            if user_res.len() == 0 {
                return ApiResponse {
                    json: Some(Json(LoginResponse {
                        success: false,
                        name: "".to_string(),
                        email: email,
                        privilege: -1,
                    })),
                    status: Status::InternalServerError,
                };
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
        })
    ).unwrap();
    response
}

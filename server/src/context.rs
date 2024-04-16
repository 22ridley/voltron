use crate::backend::MySqlBackend;
use crate::config::Config;
use alohomora::pure::PrivacyPureRegion;
use alohomora::rocket::{BBoxRequest, BBoxRequestOutcome, FromBBoxRequest};
use alohomora::AlohomoraType;
use alohomora::{bbox::BBox, policy::NoPolicy};
use alohomora::db::{BBoxConn, BBoxOpts};
use rocket::State;
use rocket_firebase_auth::{BearerToken, FirebaseAuth};
use std::convert::TryFrom;
use std::{sync::Arc, sync::Mutex};
use std::boxed::Box;

// Custom developer defined payload attached to every context.
#[derive(AlohomoraType, Clone)]
#[alohomora_out_type(verbatim = [config])]
pub struct ContextDataType {
    pub user: Option<BBox<String, NoPolicy>>,
    pub db: Arc<Mutex<BBoxConn>>,
    pub config: Config,
}

async fn firebase_auth_helper(token: String, firebase_auth: &FirebaseAuth) -> Option<String> {
    match BearerToken::try_from(token.as_str()) {
        Err(_) => None,
        Ok(token) => match firebase_auth.verify(token.as_str()).await {
            Err(_) => None,
            Ok(token) => Some(token.email.unwrap()),
        },
    }
}

// Build the custom payload for the context given HTTP request.
#[rocket::async_trait]
impl<'a, 'r> FromBBoxRequest<'a, 'r> for ContextDataType {
    type BBoxError = ();

    async fn from_bbox_request(
        request: BBoxRequest<'a, 'r>,
    ) -> BBoxRequestOutcome<Self, Self::BBoxError> {
        let config: &State<Config> = request.guard().await.unwrap();
        let firebase_auth: &State<FirebaseAuth> = request.guard().await.unwrap();

        // Connect to the DB.
        let mut db = BBoxConn::new(
            // this is the user and password from the config.toml file
            BBoxOpts::from_url(&format!("mysql://{}:{}@127.0.0.1/", config.db_user, config.db_password)).unwrap(),
        ).unwrap();
        db.query_drop("USE users").unwrap();


        // Get token from headers
        let token = request
            .headers()
            .get_one::<NoPolicy>("Authorization")
            .map(|token| {
                token.into_ppr(PrivacyPureRegion::new(|token| {
                    firebase_auth_helper(token, &firebase_auth)
                }))
            });

        let user = match token {
            None => None,
            Some(bbox) => bbox.await.transpose(),
        };

        // Return resulting context.
        BBoxRequestOutcome::Success(ContextDataType {
            user,
            db: Arc::new(Mutex::new(db)),
            config: config.inner().clone(),
        })
    }
}

use alohomora::rocket::{BBoxRequest, BBoxRequestOutcome, FromBBoxRequest};
use alohomora::AlohomoraType;
use rocket::http::Status;
use rocket::outcome::IntoOutcome;
use rocket::State;
use alohomora::{bbox::BBox, policy::NoPolicy};
use rocket_firebase_auth::{BearerToken, FirebaseAuth, FirebaseToken};
use crate::config::Config;
use crate::backend::MySqlBackend;
use std::{sync::Arc, sync::Mutex};

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

        // Firebase auth
        let firebase_auth: FirebaseAuth = FirebaseAuth::builder()
            .json_file("src/firebase-credentials.json")
            .build()
            .expect("Failed to read firebase credentials");

        // Get token from headers
        let bearer_token: BearerToken = request
            .token()
            .and_then(|token| BearerToken::try_from(token).ok()).unwrap();
        let token: Option<FirebaseToken> = match firebase_auth.verify(bearer_token.as_str()).await {
            Ok(firebase_token) => Some(firebase_token),
            Err(_) => None
        };
        // Get user email from token
        // Replace with firebase token stuff
        let user = match token {
            None => None,
            Some(tok) => {
                Some(BBox::new(tok.email.unwrap(), NoPolicy{}))
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
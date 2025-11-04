use crate::config::Config;
use rocket::State;
use rocket_firebase_auth::{FirebaseAuth, FirebaseToken};
use sesame::pcon::PCon;
use sesame::policy::NoPolicy;
use sesame::verified::VerifiedRegion;
use sesame::SesameType;
use sesame_mysql::{PConOpts, SesameConn};
use sesame_rocket::rocket::{FromPConRequest, PConRequest, PConRequestOutcome};
use std::boxed::Box;
use std::{sync::Arc, sync::Mutex};

// Custom developer defined payload attached to every context.
#[derive(SesameType)]
#[sesame_out_type(verbatim = [config])]
pub struct ContextDataType {
    pub user: Option<PCon<String, NoPolicy>>,
    pub db: Arc<Mutex<SesameConn>>,
    pub config: Config,
}
impl Clone for ContextDataType {
    fn clone(&self) -> Self {
        // Connect to the DB.
        let mut db = SesameConn::new(
            // this is the user and password from the config.toml file
            PConOpts::from_url(&format!(
                "mysql://{}:{}@127.0.0.1/",
                self.config.db_user, self.config.db_password
            ))
            .unwrap(),
        )
        .unwrap();
        db.query_drop("USE users").unwrap(); // Connect to the DB.

        Self {
            user: self.user.clone(),
            db: Arc::new(Mutex::new(db)),
            config: self.config.clone(),
        }
    }
}

// Build the custom payload for the context given HTTP request.
#[rocket::async_trait]
impl<'a, 'r> FromPConRequest<'a, 'r> for ContextDataType {
    type PConError = ();

    async fn from_pcon_request(
        request: PConRequest<'a, 'r>,
    ) -> PConRequestOutcome<Self, Self::PConError> {
        let config: &State<Config> = request.guard().await.unwrap();
        let firebase_auth: &State<FirebaseAuth> = request.guard().await.unwrap();

        // Connect to the DB.
        let mut db = SesameConn::new(
            // this is the user and password from the config.toml file
            PConOpts::from_url(&format!(
                "mysql://{}:{}@127.0.0.1/",
                config.db_user, config.db_password
            ))
            .unwrap(),
        )
        .unwrap();
        db.query_drop("USE users").unwrap();

        // Get token from headers
        let token = request.firebase_token(&firebase_auth).await;

        let user = match token {
            None => None,
            Some(token) => Some(token.into_verified(VerifiedRegion::new(
                |token: FirebaseToken| token.email.unwrap(),
            ))),
        };

        // Return resulting context.
        PConRequestOutcome::Success(ContextDataType {
            user,
            db: Arc::new(Mutex::new(db)),
            config: config.inner().clone(),
        })
    }
}

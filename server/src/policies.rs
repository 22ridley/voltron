use alohomora::db::from_value;
use alohomora::rocket::{BBoxRequest, BBoxRequestOutcome, FromBBoxRequest};
use alohomora::AlohomoraType;
use mysql::Value;
use rocket::http::{Cookie, Status};
use rocket::outcome::IntoOutcome;
use rocket::{Request, State};
use alohomora::context::{Context, UnprotectedContext};
use alohomora::policy::{AnyPolicy, FrontendPolicy, Policy, PolicyAnd, Reason, schema_policy, SchemaPolicy};
use alohomora::{bbox::BBox, policy::NoPolicy};
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

#[derive(Clone)]
#[schema_policy(table = "users", column = 1)]
pub struct QueryableOnly {}

// Content of answer column can only be accessed by:
//   1. The user who submitted the answer (`user_id == me`);
//   2. The admin(s) (`is me in set<admins>`);
//   3. Any student who is leading discussion for the lecture
//      (`P(me)` alter. `is me in set<P(students)>`);
impl Policy for QueryableOnly {
    fn name(&self) -> String {
        String::from("QueryableOnly")
    }

    fn check(&self, _context: &UnprotectedContext, reason: Reason) -> bool {
        match reason {
            Reason::DB(query) => query.starts_with("SELECT"),
            _ => false
        }
    }

    fn join(&self, other: AnyPolicy) -> Result<AnyPolicy, ()> {
        if other.is::<QueryableOnly>() { // Policies are combinable
            Ok(other)
        } else {                         //Policies must be stacked
            Ok(AnyPolicy::new(
                PolicyAnd::new(
                    self.clone(),
                    other
                )
            ))
        }
    }

    fn join_logic(&self, _other: Self) -> Result<Self, ()> {
        Ok(QueryableOnly {})
    }
}

impl FrontendPolicy for QueryableOnly {
    fn from_request<'a, 'r>(_request: &'a Request<'r>) -> Self {
        QueryableOnly {}
    }
    fn from_cookie<'a, 'r>(_name: &str, _cookie: &'a Cookie<'static>, _request: &'a Request<'r>) -> Self {
        QueryableOnly {}
    }
}

impl SchemaPolicy for QueryableOnly {
    fn from_row(_table: &str, _row: &Vec<Value>) -> Self where Self: Sized {
        QueryableOnly {}
    }
}
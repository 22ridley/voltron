use rocket::http::Cookie;
use rocket::Request;
use serde::Serialize;
use sesame::context::UnprotectedContext;
use sesame::policy::{Reason, SimplePolicy};
use sesame_rocket::policy::FrontendPolicy;

#[derive(Clone, Serialize, Debug)]
pub struct AuthStatePolicy {}

impl AuthStatePolicy {
    pub fn new() -> AuthStatePolicy {
        AuthStatePolicy {}
    }
}

impl FrontendPolicy for AuthStatePolicy {
    fn from_request(_request: &rocket::Request<'_>) -> Self {
        // Set the fields in the instructor policy
        AuthStatePolicy::new()
    }

    fn from_cookie<'a, 'r>(
        _name: &str,
        _cookie: &'a Cookie<'static>,
        _request: &'a Request<'r>,
    ) -> Self {
        AuthStatePolicy::new()
    }
}

// Email from token can only be used in database, not returned from endpoint
impl SimplePolicy for AuthStatePolicy {
    fn simple_name(&self) -> String {
        format!("AuthStatePolicy")
    }

    fn simple_check(&self, _context: &UnprotectedContext, reason: Reason) -> bool {
        // Only approve use in database queries
        match reason {
            Reason::DB(_, _) => true,
            _ => false,
        }
    }

    fn simple_join_direct(&mut self, _other: &mut Self) {}
}

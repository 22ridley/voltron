use alohomora::context::UnprotectedContext;
use alohomora::policy::{AnyPolicy, FrontendPolicy, Policy, PolicyAnd, Reason};
use rocket::http::Cookie;
use rocket::Request;
use serde::Serialize;

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
impl Policy for AuthStatePolicy {
    fn name(&self) -> String {
        format!("AuthStatePolicy")
    }

    fn check(&self, _context: &UnprotectedContext, reason: Reason) -> bool {
        // Only approve use in database queries
        match reason {
            Reason::DB(_, _) => return true,
            _ => return false,
        }
    }

    fn join(&self, other: AnyPolicy) -> Result<AnyPolicy, ()> {
        if other.is::<AuthStatePolicy>() {
            // Policies are combinable
            let other = other.specialize::<AuthStatePolicy>().unwrap();
            Ok(AnyPolicy::new(self.join_logic(other)?))
        } else {
            //Policies must be stacked
            Ok(AnyPolicy::new(PolicyAnd::new(
                AnyPolicy::new(self.clone()),
                other,
            )))
        }
    }

    fn join_logic(&self, _p2: Self) -> Result<Self, ()> {
        Ok(AuthStatePolicy {})
    }
}
use std::sync::{Arc, Mutex};
use alohomora::AlohomoraType;
use alohomora::context::{Context, UnprotectedContext};
use alohomora::policy::{schema_policy, AnyPolicy, FrontendPolicy, Policy, PolicyAnd, Reason, SchemaPolicy};
use rocket::http::Cookie;
use rocket::Request;
use serde::Serialize;
use crate::backend::MySqlBackend;
use crate::config::Config;
use crate::context::ContextDataType;

// Access control policy.
// #[schema_policy(table = "users", column = 0)]
// #[schema_policy(table = "users", column = 1)]
// #[schema_policy(table = "users", column = 2)]
// #[schema_policy(table = "users", column = 3)]
// #[schema_policy(table = "users", column = 4)]
// We can add multiple #[schema_policy(...)] definitions
// here to reuse the policy across tables/columns.
#[derive(Clone, Serialize, Debug)]
pub struct InstructorPolicy {
}

impl InstructorPolicy {
    pub fn new() -> InstructorPolicy {
        InstructorPolicy {}
    }
}

impl FrontendPolicy for InstructorPolicy {
    fn from_request(request: &rocket::Request<'_>) -> Self {
        // Set the fields in the instructor policy
        InstructorPolicy::new()
    }

    fn from_cookie<'a, 'r>(_name: &str, _cookie: &'a Cookie<'static>, _request: &'a Request<'r>) -> Self {
        InstructorPolicy::new()
    }
}

// Content of a buffer can only be accessed by:
//   1. Students with group_id and class_id;
//   2. Instructors with class_id;
impl Policy for InstructorPolicy {
    fn name(&self) -> String {
        format!("InstructorPolicy")
    }

    fn check(&self, context: &UnprotectedContext, _reason: Reason) -> bool {
        return false;
    }

    fn join(&self, other: AnyPolicy) -> Result<AnyPolicy, ()> {
        if other.is::<InstructorPolicy>() { // Policies are combinable
            let other = other.specialize::<InstructorPolicy>().unwrap();
            Ok(AnyPolicy::new(self.join_logic(other)?))
        } else {                    //Policies must be stacked
            Ok(AnyPolicy::new(
                PolicyAnd::new(
                    AnyPolicy::new(self.clone()),
                    other)))
        }
    }

    fn join_logic(&self, p2: Self) -> Result<Self, ()> {
        unimplemented!()
    }
}
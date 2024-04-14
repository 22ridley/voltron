use crate::backend::MySqlBackend;
use crate::config::Config;
use crate::context::ContextDataType;
use alohomora::context::{Context, UnprotectedContext};
use alohomora::policy::{
    schema_policy, AnyPolicy, FrontendPolicy, Policy, PolicyAnd, Reason, SchemaPolicy,
};
use alohomora::AlohomoraType;
use rocket::http::Cookie;
use rocket::Request;
use serde::Serialize;
use std::sync::{Arc, Mutex};

// Access control policy.
// #[schema_policy(table = "users", column = 0)]
// #[schema_policy(table = "users", column = 1)]
// #[schema_policy(table = "users", column = 2)]
// #[schema_policy(table = "users", column = 3)]
// #[schema_policy(table = "users", column = 4)]
// We can add multiple #[schema_policy(...)] definitions
// here to reuse the policy across tables/columns.
#[derive(Clone, Serialize, Debug)]
pub struct InstructorPolicy {}

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

    fn from_cookie<'a, 'r>(
        _name: &str,
        _cookie: &'a Cookie<'static>,
        _request: &'a Request<'r>,
    ) -> Self {
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

    fn check(&self, context: &UnprotectedContext, reason: Reason) -> bool {
        // Check if the Reason involves the database (match on Reason, anything other than DB is false)
        match reason {
            Reason::DB(_) => (),
            _ => return false,
        }

        // If they are the admin (by checking database)
        type ContextDataOut = <ContextDataType as AlohomoraType>::Out;
        let context: &ContextDataOut = context.downcast_ref().unwrap();
        let user: &Option<String> = &context.user;
        let db: &Arc<Mutex<MySqlBackend>> = &context.db;
        let user: String = user.as_ref().unwrap().to_string();
        // Check the database
        let mut bg = db.lock().unwrap();
        let admin_res = (*bg).prep_exec(
            "SELECT * FROM users WHERE email = ? AND privilege = ?",
            vec![user.clone(), "2".to_string()],
            Context::empty(),
        );
        drop(bg);

        // I am the admin.
        if admin_res.len() > 0 {
            return true;
        }
        return false;
    }

    fn join(&self, other: AnyPolicy) -> Result<AnyPolicy, ()> {
        if other.is::<InstructorPolicy>() {
            // Policies are combinable
            let other = other.specialize::<InstructorPolicy>().unwrap();
            Ok(AnyPolicy::new(self.join_logic(other)?))
        } else {
            //Policies must be stacked
            Ok(AnyPolicy::new(PolicyAnd::new(
                AnyPolicy::new(self.clone()),
                other,
            )))
        }
    }

    fn join_logic(&self, p2: Self) -> Result<Self, ()> {
        unimplemented!()
    }
}

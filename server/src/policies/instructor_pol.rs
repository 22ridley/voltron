use crate::context::ContextDataType;
use crate::mysql::prelude::Queryable;
use alohomora::context::UnprotectedContext;
use alohomora::policy::{AnyPolicy, FrontendPolicy, Policy, PolicyAnd, Reason};
use alohomora::AlohomoraType;
use rocket::http::Cookie;
use rocket::Request;
use serde::Serialize;

#[derive(Clone, Serialize, Debug)]
pub struct InstructorPolicy {}

impl InstructorPolicy {
    pub fn new() -> InstructorPolicy {
        InstructorPolicy {}
    }
}

impl FrontendPolicy for InstructorPolicy {
    fn from_request(_request: &rocket::Request<'_>) -> Self {
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

// Only admin can register instructors
impl Policy for InstructorPolicy {
    fn name(&self) -> String {
        format!("InstructorPolicy")
    }

    fn check(&self, context: &UnprotectedContext, reason: Reason) -> bool {
        // Check if the Reason involves the database (match on Reason, anything other than DB is false)
        match reason {
            Reason::DB(_, _) => (),
            _ => return false,
        }

        // If they are the admin (by checking database)
        type ContextDataOut = <ContextDataType as AlohomoraType>::Out;
        let context: &ContextDataOut = context.downcast_ref().unwrap();
        let mut db = context.db.lock().unwrap();
        let user: &Option<String> = &context.user;
        let user: String = user.as_ref().unwrap().to_string();

        // Check the database
        let mut admin_res = db
            .exec_iter(
                "SELECT * FROM user WHERE email = ? AND privilege = 2",
                (user.clone(),),
            )
            .unwrap();

        // I am the admin.
        if let None = admin_res.next() {
            return false;
        }
        return true;
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

    fn join_logic(&self, _p2: Self) -> Result<Self, ()> {
        Ok(InstructorPolicy {})
    }
}

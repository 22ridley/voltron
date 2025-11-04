use crate::context::ContextDataType;
use crate::mysql::prelude::Queryable;
use rocket::http::Cookie;
use rocket::Request;
use serde::Serialize;
use sesame::context::UnprotectedContext;
use sesame::policy::{Reason, SimplePolicy};
use sesame::SesameTypeOut;
use sesame_rocket::policy::FrontendPolicy;

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
impl SimplePolicy for InstructorPolicy {
    fn simple_name(&self) -> String {
        format!("InstructorPolicy")
    }

    fn simple_check(&self, context: &UnprotectedContext, reason: Reason) -> bool {
        // Check if the Reason involves the database (match on Reason, anything other than DB is false)
        match reason {
            Reason::DB(_, _) => (),
            _ => return false,
        }

        // If they are the admin (by checking database)
        type ContextDataOut = <ContextDataType as SesameTypeOut>::Out;
        let context: &ContextDataOut = context.downcast_ref().unwrap();
        let mut db = context.db.lock().unwrap();
        let user: &Option<String> = &context.user;
        let user: String = user.as_ref().unwrap().to_string();

        // Check the database
        let mut admin_res = db
            .exec_iter(
                "SELECT * FROM users WHERE email = ? AND privilege = 2",
                (user.clone(),),
            )
            .unwrap();

        // I am the admin.
        if let None = admin_res.next() {
            return false;
        }
        return true;
    }

    fn simple_join_direct(&mut self, _other: &mut Self) {}
}

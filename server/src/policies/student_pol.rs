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
pub struct StudentPolicy {}

impl StudentPolicy {
    pub fn new() -> StudentPolicy {
        StudentPolicy {}
    }
}

impl FrontendPolicy for StudentPolicy {
    fn from_request(_request: &rocket::Request<'_>) -> Self {
        // Set the fields in the instructor policy
        StudentPolicy::new()
    }

    fn from_cookie<'a, 'r>(
        _name: &str,
        _cookie: &'a Cookie<'static>,
        _request: &'a Request<'r>,
    ) -> Self {
        StudentPolicy::new()
    }
}

// Only instructors can register students for their own class
impl SimplePolicy for StudentPolicy {
    fn simple_name(&self) -> String {
        format!("StudentPolicy")
    }

    fn simple_check(&self, context: &UnprotectedContext, reason: Reason) -> bool {
        // Check if the Reason involves the database (match on Reason, anything other than DB is false)
        match reason {
            Reason::DB(_query, params) => {
                // If they are an instructor (by checking database)
                type ContextDataOut = <ContextDataType as SesameTypeOut>::Out;
                let context: &ContextDataOut = context.downcast_ref().unwrap();
                let mut db = context.db.lock().unwrap();
                let user: &Option<String> = &context.user;
                let user: String = user.as_ref().unwrap().to_string();

                // Check the database
                let mut instructor_res = db
                    .exec_iter(
                        "SELECT * FROM users WHERE email = ? AND privilege = 1",
                        (user.clone(),),
                    )
                    .unwrap();

                // Get the instructor's class_id
                let row = instructor_res.next().unwrap().unwrap();
                let instructor_class: i32 = mysql::from_value(row[3].clone());

                // Get the class_id that the instructor is trying to put a student into
                let query_class_id: i32 = mysql::from_value(params[3].clone());

                // Fail if the instructor is trying to place a student into a class that is not the instructor's class
                if instructor_class != query_class_id {
                    return false;
                }
                true
            }
            Reason::Custom(_) => true,
            _ => false,
        }
    }

    fn simple_join_direct(&mut self, _other: &mut Self) {}
}

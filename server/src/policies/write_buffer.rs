use crate::context::ContextDataType;
use alohomora::context::UnprotectedContext;
use alohomora::policy::{AnyPolicy, CloneableAny, FrontendPolicy, Policy, Reason};
use alohomora::AlohomoraType;
use mysql::prelude::Queryable;
use rocket::http::Cookie;
use rocket::Request;
use serde::Serialize;

#[derive(Clone, Serialize, Debug)]
pub struct WriteBufferPolicy {}

// Content of a buffer can only be accessed by:
//   1. Students with group_id and class_id;
//   2. Instructors with class_id;
//   3. Admins
impl Policy for WriteBufferPolicy {
    fn name(&self) -> String {
        String::from("WriteBufferPolicy")
    }

    fn check(&self, context: &UnprotectedContext, reason: Reason) -> bool {
        let (class_id, group_id): (i32, i32) = match reason {
            Reason::Custom(arg) => {
                let arg = &*arg;
                let tup: &(i32, i32) = arg.cast().downcast_ref().unwrap();
                (tup.0, tup.1)
            },
            _ => { return false; },
        };

        type ContextDataOut = <ContextDataType as AlohomoraType>::Out;
        let context: &ContextDataOut = context.downcast_ref().unwrap();

        let user: &Option<String> = &context.user;
        let mut db = context.db.lock().unwrap();

        // I am not an authenticated user. I cannot wrtie to any buffers!
        if user.is_none() {
            return false;
        }

        let user: &String = user.as_ref().unwrap();

        // Check the database
        let mut result = db
            .exec_iter(
                "SELECT * FROM users WHERE email = ? AND clasS_id = ? AND group_id = ?",
                (user, class_id, group_id)
            )
            .unwrap();

        // Find out if we are an instructor for the class, or a student in the class and group.
        match result.next() {
            None => false,
            Some(res) => true,
        }
    }

    fn join(&self, other: AnyPolicy) -> Result<AnyPolicy, ()> {
        todo!()
    }

    fn join_logic(&self, p2: Self) -> Result<Self, ()> {
        todo!()
    }
}

impl FrontendPolicy for WriteBufferPolicy {
    fn from_cookie<'a, 'r>(name: &str, cookie: &'a Cookie<'static>, request: &'a Request<'r>) -> Self where Self: Sized {
        WriteBufferPolicy {}
    }
    fn from_request<'a, 'r>(request: &'a Request<'r>) -> Self where Self: Sized {
        WriteBufferPolicy {}
    }
}

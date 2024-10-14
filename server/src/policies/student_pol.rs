use crate::context::ContextDataType;
use crate::mysql::prelude::Queryable;
use alohomora::context::UnprotectedContext;
use alohomora::policy::{AnyPolicy, FrontendPolicy, Policy, PolicyAnd, Reason};
use alohomora::AlohomoraType;
use rocket::http::Cookie;
use rocket::Request;
use serde::Serialize;

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
impl Policy for StudentPolicy {
    fn name(&self) -> String {
        format!("StudentPolicy")
    }

    fn check(&self, context: &UnprotectedContext, reason: Reason) -> bool {
        // Check if the Reason involves the database (match on Reason, anything other than DB is false)
        match reason {
            Reason::DB(query, params) => {
                // If they are an instructor (by checking database)
                type ContextDataOut = <ContextDataType as AlohomoraType>::Out;
                let context: &ContextDataOut = context.downcast_ref().unwrap();
                let mut db = context.db.lock().unwrap();
                let user: &Option<String> = &context.user;
                let user: String = user.as_ref().unwrap().to_string();

                let split: Vec<&str> = query.split_whitespace().collect();
                let mut table_name: &str = "";
                if let Some(index) = split.iter().position(|&word| word == "INTO") {
                    if index + 1 < split.len() {
                        table_name = split[index + 1];
                    }
                }
                println!("{}", table_name);

                if table_name == "enroll" || table_name == "`group`" {
                    // Check the database
                    let mut instructor_res = db
                        .exec_iter(
                            "SELECT class_id FROM user_class WHERE email = ? AND privilege = 1",
                            (user.clone(),),
                        )
                        .unwrap();

                    // Get the instructor's class_id
                    let row = instructor_res.next().unwrap().unwrap();
                    let instructor_class: i32 = mysql::from_value(row[0].clone());

                    // Get the class_id that the instructor is trying to enroll a student into
                    let query_class_id: i32 = mysql::from_value(params[1].clone());

                    // Fail if the instructor is trying to place a student into a class that is not the instructor's class
                    if instructor_class != query_class_id {
                        return false;
                    }
                    return true;
                } else if table_name == "user" {
                    // Get the email of the student that the instructor is trying to add
                    let privilege: String = mysql::from_value(params[2].clone());
                    if privilege != "0" {
                        return false;
                    }
                    return true;
                } else {
                    return false;
                }
            }
            Reason::Custom(_) => true,
            _ => return false,
        }
    }

    fn join(&self, other: AnyPolicy) -> Result<AnyPolicy, ()> {
        if other.is::<StudentPolicy>() {
            // Policies are combinable
            let other = other.specialize::<StudentPolicy>().unwrap();
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
        Ok(StudentPolicy {})
    }
}

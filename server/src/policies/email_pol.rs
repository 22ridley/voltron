use crate::context::ContextDataType;
use serde::Serialize;
use sesame::context::UnprotectedContext;
use sesame::policy::{Reason, SimplePolicy};
use sesame::SesameTypeOut;
use sesame_mysql::{schema_policy, SchemaPolicy};

#[schema_policy(table = "users", column = 1)]
#[derive(Clone, Serialize, Debug)]
pub struct EmailPolicy {
    email: String,
}

impl EmailPolicy {
    pub fn new(email: String) -> EmailPolicy {
        EmailPolicy { email }
    }
}

// Only admin can register instructors
impl SimplePolicy for EmailPolicy {
    fn simple_name(&self) -> String {
        format!("EmailPolicy")
    }

    fn simple_check(&self, context: &UnprotectedContext, _reason: Reason) -> bool {
        // Make sure that this is their own email
        type ContextDataOut = <ContextDataType as SesameTypeOut>::Out;
        let context: &ContextDataOut = context.downcast_ref().unwrap();
        let user: &Option<String> = &context.user;
        let user: String = user.as_ref().unwrap().to_string();

        if user == self.email {
            return true;
        } else {
            return false;
        }
    }

    fn simple_join_direct(&mut self, other: &mut Self) {
        if self.email != other.email {
            panic!("Cannot join different emails");
        }
    }
}

impl SchemaPolicy for EmailPolicy {
    fn from_row(_table: &str, row: &Vec<mysql::Value>) -> Self
    where
        Self: Sized,
    {
        EmailPolicy::new(mysql::from_value(row[1].clone()))
    }
}

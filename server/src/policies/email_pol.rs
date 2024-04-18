use crate::context::ContextDataType;
use alohomora::context::UnprotectedContext;
use alohomora::policy::{AnyPolicy, Policy, PolicyAnd, Reason, SchemaPolicy};
use alohomora::AlohomoraType;
use serde::Serialize;

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
impl Policy for EmailPolicy {
    fn name(&self) -> String {
        format!("EmailPolicy")
    }

    fn check(&self, context: &UnprotectedContext, _reason: Reason) -> bool {
        // Make sure that this is their own email
        type ContextDataOut = <ContextDataType as AlohomoraType>::Out;
        let context: &ContextDataOut = context.downcast_ref().unwrap();
        let user: &Option<String> = &context.user;
        let user: String = user.as_ref().unwrap().to_string();

        if user == self.email {
            return true;
        } else {
            return false;
        }
    }

    fn join(&self, other: AnyPolicy) -> Result<AnyPolicy, ()> {
        if other.is::<EmailPolicy>() {
            // Policies are combinable
            let other = other.specialize::<EmailPolicy>().unwrap();
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
        if self.email == p2.email {
            Ok(EmailPolicy {
                email: self.email.clone(),
            })
        } else {
            Err(())
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

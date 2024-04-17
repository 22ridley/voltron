use crate::context::ContextDataType;
use alohomora::context::UnprotectedContext;
use alohomora::policy::{AnyPolicy, Policy, PolicyAnd, Reason, SchemaPolicy};
use alohomora::AlohomoraType;
use mysql::prelude::Queryable;
use serde::Serialize;

// Access control policy.
// #[schema_policy(table = "users", column = 0)]
// #[schema_policy(table = "users", column = 1)]
// #[schema_policy(table = "users", column = 2)]
// #[schema_policy(table = "users", column = 3)]
// #[schema_policy(table = "users", column = 4)]
// We can add multiple #[schema_policy(...)] definitions
// here to reuse the policy across tables/columns.
#[derive(Clone, Serialize, Debug)]
pub struct VoltronBufferPolicy {
    class_id: i32, // Only students in the proper group in the proper class can access this buffer
    group_id: i32, // Instructors for this class can also access this buffer
}

impl VoltronBufferPolicy {
    pub fn new(class_id: i32, group_id: i32) -> VoltronBufferPolicy {
        VoltronBufferPolicy { class_id, group_id }
    }
}

// Content of a buffer can only be accessed by:
//   1. Students with group_id and class_id;
//   2. Instructors with class_id;
//   3. Admins
impl Policy for VoltronBufferPolicy {
    fn name(&self) -> String {
        format!(
            "VoltronBufferPolicy(class id {:?} and group id {:?})",
            self.class_id, self.group_id
        )
    }

    fn check(&self, context: &UnprotectedContext, _reason: Reason) -> bool {
        type ContextDataOut = <ContextDataType as AlohomoraType>::Out;
        let context: &ContextDataOut = context.downcast_ref().unwrap();

        let user: &Option<String> = &context.user;
        let mut db = context.db.lock().unwrap();

        // I am not an authenticated user. I cannot see any buffers!
        if user.is_none() {
            return false;
        }

        let user: &String = user.as_ref().unwrap();

        // Check the database
        let mut result = db
            .exec_iter("SELECT * FROM users WHERE email = ?", (user,))
            .unwrap();

        // Find out if we are an instructor for the class, or a student in the class and group.
        match result.next() {
            None => false,
            Some(res) => {
                match res {
                    Err(_) => false,
                    Ok(row) => {
                        let privilege: i32 = mysql::from_value(row.get(2).unwrap());
                        let class_id: i32 = mysql::from_value(row.get(3).unwrap());
                        let group_id: i32 = mysql::from_value(row.get(4).unwrap());
                        if privilege == 2 {
                            // I am an admin
                            true
                        } else if privilege == 1 && class_id == self.class_id {
                            // I am an instructor of this class.
                            true
                        } else if privilege == 0
                            && class_id == self.class_id
                            && group_id == self.group_id
                        {
                            // I am a student in this class and group.
                            true
                        } else {
                            false
                        }
                    }
                }
            }
        }
    }

    fn join(&self, other: AnyPolicy) -> Result<AnyPolicy, ()> {
        println!("Trying to join buffer pols {}", other.name());
        if other.is::<VoltronBufferPolicy>() {
            println!("case1");
            // Policies are combinable
            let other = other.specialize::<VoltronBufferPolicy>().unwrap();
            Ok(AnyPolicy::new(self.join_logic(other)?))
        } else {
            println!("case2");
            //Policies must be stacked
            Ok(AnyPolicy::new(PolicyAnd::new(
                AnyPolicy::new(self.clone()),
                other,
            )))
        }
    }

    fn join_logic(&self, p2: Self) -> Result<Self, ()> {
        let comp_class_id: i32;
        let comp_group_id: i32;
        if self.class_id == p2.class_id {
            comp_class_id = self.class_id;
        } else {
            comp_class_id = -10;
        }
        if self.group_id == p2.group_id {
            comp_group_id = self.group_id;
        } else {
            comp_group_id = -10;
        }
        println!(
            "Successfully joined policies: class_id = {} and group_id = {}",
            comp_class_id, comp_group_id
        );
        Ok(VoltronBufferPolicy {
            class_id: comp_class_id,
            group_id: comp_group_id,
        })
    }
}

impl SchemaPolicy for VoltronBufferPolicy {
    fn from_row(_table: &str, row: &Vec<mysql::Value>) -> Self
    where
        Self: Sized,
    {
        VoltronBufferPolicy::new(
            // class_id
            mysql::from_value(row[3].clone()),
            // group_id
            mysql::from_value(row[4].clone()),
        )
    }
}

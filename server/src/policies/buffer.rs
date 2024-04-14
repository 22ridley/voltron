use crate::backend::MySqlBackend;
use crate::context::ContextDataType;
use alohomora::context::{Context, UnprotectedContext};
use alohomora::policy::{AnyPolicy, Policy, PolicyAnd, Reason, SchemaPolicy};
use alohomora::AlohomoraType;
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
pub struct VoltronBufferPolicy {
    class_id: Option<i32>, // Only students in the proper group in the proper class can access this buffer
    group_id: Option<i32>,
    // Instructors for this class can also access this buffer
}

impl VoltronBufferPolicy {
    pub fn new(class_id: Option<i32>, group_id: Option<i32>) -> VoltronBufferPolicy {
        VoltronBufferPolicy { class_id, group_id }
    }
}

// Content of a buffer can only be accessed by:
//   1. Students with group_id and class_id;
//   2. Instructors with class_id;
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
        let db: &Arc<Mutex<MySqlBackend>> = &context.db;

        // I am not an authenticated user. I cannot see any buffers!
        if user.is_none() {
            return false;
        }

        let user: String = user.as_ref().unwrap().to_string();
        let class_id: i32 = self.class_id.unwrap();
        let group_id: i32 = self.group_id.unwrap();
        // Check the database
        let mut bg = db.lock().unwrap();
        let student_res = (*bg).prep_exec(
            "SELECT * FROM users WHERE email = ? AND class_id = ? AND group_id = ?",
            vec![user.clone(), class_id.to_string(), group_id.to_string()],
            Context::empty(),
        );
        let instr_res = (*bg).prep_exec(
            "SELECT * FROM users WHERE email = ? AND class_id = ? AND group_id = ?",
            vec![user.clone(), class_id.to_string(), "-1".to_string()],
            Context::empty(),
        );
        drop(bg);

        // I am a student in this class and group.
        if student_res.len() > 0 {
            return true;
        }

        // I am an instructor of this class.
        if instr_res.len() > 0 {
            return true;
        }

        return false;
    }

    fn join(&self, other: AnyPolicy) -> Result<AnyPolicy, ()> {
        if other.is::<VoltronBufferPolicy>() {
            // Policies are combinable
            let other = other.specialize::<VoltronBufferPolicy>().unwrap();
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
        let comp_class_id: Option<i32>;
        let comp_group_id: Option<i32>;
        if self.class_id.eq(&p2.class_id) {
            comp_class_id = self.class_id.clone();
        } else {
            comp_class_id = None;
        }
        if self.group_id.eq(&p2.group_id) {
            comp_group_id = self.group_id.clone();
        } else {
            comp_group_id = None;
        }
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

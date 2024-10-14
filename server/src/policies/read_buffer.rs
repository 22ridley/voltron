use crate::context::ContextDataType;
use alohomora::context::UnprotectedContext;
use alohomora::policy::{AnyPolicy, Policy, PolicyAnd, Reason, SchemaPolicy};
use alohomora::AlohomoraType;
use mysql::prelude::Queryable;
use serde::Serialize;

#[derive(Clone, Serialize, Debug)]
pub struct ReadBufferPolicy {
    class_id: i32, // Only students in the proper group in the proper class can access this buffer
    group_id: i32, // Instructors for this class can also access this buffer
}

impl ReadBufferPolicy {
    pub fn new(class_id: i32, group_id: i32) -> ReadBufferPolicy {
        ReadBufferPolicy { class_id, group_id }
    }
}

// Content of a buffer can only be accessed by:
//   1. Students with group_id and class_id;
//   2. Instructors with class_id;
//   3. Admins
impl Policy for ReadBufferPolicy {
    fn name(&self) -> String {
        format!(
            "ReadBufferPolicy(class id {:?} and group id {:?})",
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
        let mut student = false;

        {
            // Check the database
            let mut result = db
                .exec_iter("SELECT * FROM user WHERE email = ?", (user,))
                .unwrap();

            // Find out if we are an instructor for the class, or a student in the class and group.
            match result.next() {
                None => {return false;},
                Some(res) => {
                    match res {
                        Err(_) => {return false;},
                        Ok(row) => {
                            let privilege: i32 = mysql::from_value(row.get(3).unwrap());
                            if privilege == 2 {
                                // I am an admin
                                return true;
                            } else if privilege == 0 {
                                student = true;
                            } 
                        }
                    }
                }
            }
        }

        if student {
            let mut result = db
                .exec_iter("SELECT class_id, group_id FROM user_enroll WHERE email = ?", (user,))
                .unwrap();
            let row = result.next().unwrap().unwrap();
            let class_id: i32 = mysql::from_value(row.get(0).unwrap());
            let group_id: i32 = mysql::from_value(row.get(1).unwrap());
            // I am a student in this class and group.
            class_id == self.class_id && group_id == self.group_id
        } else {
            let mut result = db
                .exec_iter("SELECT class_id FROM user_class WHERE email = ?", (user,))
                .unwrap();
            let class_id: i32 = mysql::from_value(result.next().unwrap().unwrap().get(0).unwrap());
            // I am the instructor of this class
            class_id == self.class_id
        }
    }

    fn join(&self, other: AnyPolicy) -> Result<AnyPolicy, ()> {
        if other.is::<ReadBufferPolicy>() {
            // Policies are combinable
            let other = other.specialize::<ReadBufferPolicy>().unwrap();
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
        let comp_class_id: i32;
        let comp_group_id: i32;
        if self.class_id == p2.class_id {
            comp_class_id = self.class_id;
        } else {
            comp_class_id = -1;
        }
        if self.group_id == p2.group_id {
            comp_group_id = self.group_id;
        } else {
            comp_group_id = -1;
        }
        Ok(ReadBufferPolicy {
            class_id: comp_class_id,
            group_id: comp_group_id,
        })
    }
}

impl SchemaPolicy for ReadBufferPolicy {
    fn from_row(_table: &str, row: &Vec<mysql::Value>) -> Self
    where
        Self: Sized,
    {
        ReadBufferPolicy::new(
            // class_id
            mysql::from_value(row[5].clone()),
            // group_id
            mysql::from_value(row[6].clone()),
        )
    }
}

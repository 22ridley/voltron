use crate::context::ContextDataType;
use mysql::prelude::Queryable;
use serde::Serialize;
use sesame::context::UnprotectedContext;
use sesame::policy::{Reason, SimplePolicy};
use sesame::SesameTypeOut;
use sesame_mysql::{schema_policy, SchemaPolicy};

#[schema_policy(table = "users", column = 3)]
#[schema_policy(table = "users", column = 4)]
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
impl SimplePolicy for ReadBufferPolicy {
    fn simple_name(&self) -> String {
        format!(
            "ReadBufferPolicy(class id {:?} and group id {:?})",
            self.class_id, self.group_id
        )
    }

    fn simple_check(&self, context: &UnprotectedContext, _reason: Reason) -> bool {
        type ContextDataOut = <ContextDataType as SesameTypeOut>::Out;
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

    fn simple_join_direct(&mut self, other: &mut Self) {
        let comp_class_id: i32;
        let comp_group_id: i32;
        if self.class_id == other.class_id {
            comp_class_id = self.class_id;
        } else {
            comp_class_id = -10;
        }
        if self.group_id == other.group_id {
            comp_group_id = self.group_id;
        } else {
            comp_group_id = -10;
        }
        self.class_id = comp_class_id;
        self.group_id = comp_group_id;
    }
}

impl SchemaPolicy for ReadBufferPolicy {
    fn from_row(_table: &str, row: &Vec<mysql::Value>) -> Self
    where
        Self: Sized,
    {
        ReadBufferPolicy::new(
            // class_id
            mysql::from_value(row[3].clone()),
            // group_id
            mysql::from_value(row[4].clone()),
        )
    }
}

use crate::backend::MySqlBackend;
use crate::context::ContextDataType;
use crate::policies::{AuthStatePolicy, ReadBufferPolicy};
use crate::routes::common::{email_from_token, read_buffer, Student, StudentGroup};
use sesame::context::Context;
use sesame::pcon::PCon;
use sesame::policy::{AnyPolicy, AnyPolicyDyn};
use sesame_mysql::from_value;
use sesame_rocket::rocket::{get, JsonResponse, ResponsePConJson};

use mysql::Value;
use rocket::State;
use rocket_firebase_auth::FirebaseToken;
use sesame::fold::fold;
use sesame::verified::VerifiedRegion;
use std::collections::{BTreeSet, HashMap};
use std::iter::FromIterator;
use std::sync::{Arc, Mutex};

#[derive(ResponsePConJson)]
pub struct InstructorResponse {
    pub success: bool,
    pub class_id: PCon<i64, ReadBufferPolicy>,
    pub students: Vec<Student>,
    pub student_groups: Vec<StudentGroup>,
}

#[get("/instructor")]
pub(crate) fn instructor(
    token: PCon<FirebaseToken, AuthStatePolicy>,
    backend: &State<Arc<Mutex<MySqlBackend>>>,
    context: Context<ContextDataType>,
) -> JsonResponse<InstructorResponse, ContextDataType> {
    // Find this instructor
    let email: PCon<String, AuthStatePolicy> = email_from_token(token);
    let mut bg: std::sync::MutexGuard<'_, MySqlBackend> = backend.lock().unwrap();
    // Get this instructor's class ID
    let user_res: Vec<Vec<PCon<Value, AnyPolicy>>> = (*bg).prep_exec(
        "SELECT * FROM users WHERE email = ?",
        vec![email.clone()],
        context.clone(),
    );

    let mut row: Vec<PCon<Value, AnyPolicy>> = user_res.into_iter().next().unwrap();
    let class_id = row.swap_remove(3);
    let class_id: PCon<i32, ReadBufferPolicy> = from_value(class_id).unwrap();

    let students_res: Vec<Vec<PCon<Value, AnyPolicy>>> = (*bg).prep_exec(
        "SELECT * FROM users WHERE privilege = 0 AND class_id = ?",
        vec![class_id.clone()],
        context.clone(),
    );
    drop(bg);

    let mut group_ids_vec: Vec<PCon<i32, ReadBufferPolicy>> = Vec::new();
    let mut students_vec: Vec<Student> = Vec::new();
    for mut row in students_res.into_iter() {
        let group_id = row.swap_remove(4);
        let student_name = row.swap_remove(0);
        let group_id: PCon<i32, ReadBufferPolicy> = from_value(group_id).unwrap();
        let student_name: PCon<String, AnyPolicy> = from_value(student_name).unwrap();
        let student: Student = Student {
            name: student_name,
            group_id: group_id.clone().into_pcon(),
        };
        students_vec.push(student);
        group_ids_vec.push(group_id);
    }

    let group_ids_vec = fold::<dyn AnyPolicyDyn, _>(group_ids_vec).unwrap();
    let group_ids_vec = group_ids_vec.into_verified(VerifiedRegion::new(|v: Vec<i32>| {
        Vec::from_iter(BTreeSet::from_iter(v.into_iter()).into_iter())
    }));
    let group_ids_vec: PCon<_, ReadBufferPolicy> = group_ids_vec.specialize_policy().unwrap();
    let group_ids_vec: Vec<_> = group_ids_vec.fold_in();

    let mut group_vec: Vec<StudentGroup> = Vec::new();
    for group_id in group_ids_vec {
        let group = StudentGroup {
            group_id: group_id.clone().into_pcon(),
            code: read_buffer(class_id.clone(), group_id, context.clone()),
        };
        group_vec.push(group);
    }

    JsonResponse::from((
        InstructorResponse {
            success: true,
            class_id: class_id.into_pcon(),
            students: students_vec,
            student_groups: group_vec,
        },
        context,
    ))
}

// Buggy version of endpoint!
#[get("/instructor_buggy")]
pub(crate) fn instructor_buggy(
    token: PCon<FirebaseToken, AuthStatePolicy>,
    backend: &State<Arc<Mutex<MySqlBackend>>>,
    context: Context<ContextDataType>,
) -> JsonResponse<InstructorResponse, ContextDataType> {
    // Find this instructor
    let email: PCon<String, AuthStatePolicy> = email_from_token(token);
    let mut bg: std::sync::MutexGuard<'_, MySqlBackend> = backend.lock().unwrap();
    // Get this instructor's class ID
    let user_res: Vec<Vec<PCon<Value, AnyPolicy>>> = (*bg).prep_exec(
        "SELECT * FROM users WHERE email = ?",
        vec![email.clone()],
        context.clone(),
    );

    let mut row: Vec<PCon<Value, AnyPolicy>> = user_res.into_iter().next().unwrap();
    let class_id = row.swap_remove(3);
    let class_id: PCon<i32, ReadBufferPolicy> = from_value(class_id).unwrap();

    // BUGGY: Instructor tries to read buffers of ALL students
    let students_res: Vec<Vec<PCon<Value, AnyPolicy>>> = (*bg).prep_exec(
        "SELECT * FROM users WHERE privilege = 0",
        vec![""],
        context.clone(),
    );
    drop(bg);

    let mut group_ids_vec: Vec<PCon<i32, ReadBufferPolicy>> = Vec::new();
    let mut students_vec: Vec<Student> = Vec::new();
    for mut row in students_res.into_iter() {
        let group_id = row.swap_remove(4);
        let student_name = row.swap_remove(0);
        let group_id: PCon<i32, ReadBufferPolicy> = from_value(group_id).unwrap();
        let student_name: PCon<String, AnyPolicy> = from_value(student_name).unwrap();
        let student: Student = Student {
            name: student_name,
            group_id: group_id.clone().into_pcon(),
        };
        students_vec.push(student);
        group_ids_vec.push(group_id);
    }

    let group_ids_vec = fold::<dyn AnyPolicyDyn, _>(group_ids_vec).unwrap();
    let group_ids_vec = group_ids_vec.into_verified(VerifiedRegion::new(|v: Vec<i32>| {
        Vec::from_iter(BTreeSet::from_iter(v.into_iter()).into_iter())
    }));
    let group_ids_vec: PCon<_, ReadBufferPolicy> = group_ids_vec.specialize_policy().unwrap();
    let group_ids_vec: Vec<_> = group_ids_vec.fold_in();

    let mut group_vec: Vec<StudentGroup> = Vec::new();
    for group_id in group_ids_vec {
        let group = StudentGroup {
            group_id: group_id.clone().into_pcon(),
            code: read_buffer(class_id.clone(), group_id, context.clone()),
        };
        group_vec.push(group);
    }

    JsonResponse::from((
        InstructorResponse {
            success: true,
            class_id: class_id.into_pcon(),
            students: students_vec,
            student_groups: group_vec,
        },
        context,
    ))
}

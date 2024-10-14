use crate::backend::MySqlBackend;
use crate::common::{email_bbox_from_token, read_buffer, Student, StudentGroup};
use crate::context::ContextDataType;
use crate::policies::{AuthStatePolicy, ReadBufferPolicy};
use alohomora::context::Context;
use alohomora::fold::fold;
use alohomora::pure::PrivacyPureRegion;
use alohomora::rocket::{get, JsonResponse, ResponseBBoxJson};
use alohomora::{bbox::BBox, db::from_value, policy::AnyPolicy};
use mysql::Value;
use rocket::State;
use rocket_firebase_auth::FirebaseToken;
use std::collections::{BTreeSet, HashMap};
use std::iter::FromIterator;
use std::sync::{Arc, Mutex};

#[derive(ResponseBBoxJson)]
pub struct InstructorResponse {
    pub success: bool,
    pub class_id: BBox<i64, AnyPolicy>,
    pub class_name: BBox<String, AnyPolicy>,
    pub students: Vec<Student>,
    pub student_groups: Vec<StudentGroup>,
}

#[get("/instructor")]
pub(crate) fn instructor(
    token: BBox<FirebaseToken, AuthStatePolicy>,
    backend: &State<Arc<Mutex<MySqlBackend>>>,
    context: Context<ContextDataType>,
) -> JsonResponse<InstructorResponse, ContextDataType> {
    // Find this instructor
    let email_bbox: BBox<String, AuthStatePolicy> = email_bbox_from_token(token);
    let mut bg: std::sync::MutexGuard<'_, MySqlBackend> = backend.lock().unwrap();
    // Get this instructor's class ID
    let user_res: Vec<Vec<BBox<Value, AnyPolicy>>> = (*bg).prep_exec(
        "SELECT * FROM user INNER JOIN class ON user.user_id = class.instructor_id WHERE email = ?",
        vec![email_bbox.clone()],
        context.clone(),
    );

    let row: Vec<BBox<Value, AnyPolicy>> = user_res[0].clone();
    let class_id_bbox: BBox<i32, AnyPolicy> = from_value(row[4].clone()).unwrap();
    let class_name_bbox: BBox<String, AnyPolicy> = from_value(row[5].clone()).unwrap();

    let students_res: Vec<Vec<BBox<Value, AnyPolicy>>> = (*bg).prep_exec(
        "SELECT * FROM user_enroll WHERE class_id = ?",
        vec![class_id_bbox.clone()],
        context.clone(),
    );
    drop(bg);

    let class_id_read_buffer_pol: BBox<i32, ReadBufferPolicy> = from_value(students_res[0].clone()[5].clone()).unwrap();
    let mut group_ids_bbox_vec: Vec<BBox<i32, ReadBufferPolicy>> = Vec::new();
    let mut students_bbox_vec: Vec<Student> = Vec::new();
    for row in students_res.iter() {
        let stud_name_bbox: BBox<String, AnyPolicy> = from_value(row[1].clone()).unwrap();
        let group_id_bbox: BBox<i32, ReadBufferPolicy> = from_value(row[6].clone()).unwrap();
        let student_bbox: Student = Student {
            name: stud_name_bbox,
            group_id: group_id_bbox.clone().into_bbox(),
        };
        students_bbox_vec.push(student_bbox);
        group_ids_bbox_vec.push(group_id_bbox);
    }

    let group_ids_bbox_vec = fold(group_ids_bbox_vec).unwrap();
    let group_ids_bbox_vec = group_ids_bbox_vec.into_ppr(PrivacyPureRegion::new(|v: Vec<i32>| {
        Vec::from_iter(BTreeSet::from_iter(v.into_iter()).into_iter())
    }));
    let group_ids_bbox_vec: BBox<_, ReadBufferPolicy> =
        group_ids_bbox_vec.specialize_policy().unwrap();
    let group_ids_bbox_vec: Vec<_> = group_ids_bbox_vec.into();

    let mut group_bbox_vec: Vec<StudentGroup> = Vec::new();
    for group_id in group_ids_bbox_vec {
        let group = StudentGroup {
            group_id: group_id.clone().into_bbox(),
            code: read_buffer(class_id_read_buffer_pol.clone(), group_id, context.clone()),
        };
        group_bbox_vec.push(group);
    }

    JsonResponse::from((
        InstructorResponse {
            success: true,
            class_id: class_id_bbox.into_bbox(),
            class_name: class_name_bbox.into_bbox(),
            students: students_bbox_vec,
            student_groups: group_bbox_vec,
        },
        context,
    ))
}

// Buggy version of endpoint!
#[get("/instructor_buggy")]
pub(crate) fn instructor_buggy(
    token: BBox<FirebaseToken, AuthStatePolicy>,
    backend: &State<Arc<Mutex<MySqlBackend>>>,
    context: Context<ContextDataType>,
) -> JsonResponse<InstructorResponse, ContextDataType> {
    // Find this instructor
    // Find this instructor
    let email_bbox: BBox<String, AuthStatePolicy> = email_bbox_from_token(token);
    let mut bg: std::sync::MutexGuard<'_, MySqlBackend> = backend.lock().unwrap();
    // Get this instructor's class ID
    let user_res: Vec<Vec<BBox<Value, AnyPolicy>>> = (*bg).prep_exec(
        "SELECT * FROM user INNER JOIN class ON user.user_id = class.instructor_id WHERE email = ?",
        vec![email_bbox.clone()],
        context.clone(),
    );

    let row: Vec<BBox<Value, AnyPolicy>> = user_res[0].clone();
    let class_id_bbox: BBox<i32, AnyPolicy> = from_value(row[4].clone()).unwrap();
    let class_name_bbox: BBox<String, AnyPolicy> = from_value(row[5].clone()).unwrap();

    // BUGGY: Instructor tries to read buffers of ALL students
    let students_res: Vec<Vec<BBox<Value, AnyPolicy>>> = (*bg).prep_exec(
        "SELECT * FROM user_enroll",
        vec![""], 
        context.clone(),
    );
    drop(bg);

    let class_id_read_buffer_pol: BBox<i32, ReadBufferPolicy> = from_value(students_res[0].clone()[5].clone()).unwrap();
    let mut group_ids_bbox_vec: Vec<BBox<i32, ReadBufferPolicy>> = Vec::new();
    let mut students_bbox_vec: Vec<Student> = Vec::new();
    for row in students_res.iter() {
        let stud_name_bbox: BBox<String, AnyPolicy> = from_value(row[1].clone()).unwrap();
        let group_id_bbox: BBox<i32, ReadBufferPolicy> = from_value(row[6].clone()).unwrap();
        let student_bbox: Student = Student {
            name: stud_name_bbox,
            group_id: group_id_bbox.clone().into_bbox(),
        };
        students_bbox_vec.push(student_bbox);
        group_ids_bbox_vec.push(group_id_bbox);
    }

    let group_ids_bbox_vec = fold(group_ids_bbox_vec).unwrap();
    let group_ids_bbox_vec = group_ids_bbox_vec.into_ppr(PrivacyPureRegion::new(|v: Vec<i32>| {
        Vec::from_iter(BTreeSet::from_iter(v.into_iter()).into_iter())
    }));
    let group_ids_bbox_vec: BBox<_, ReadBufferPolicy> =
        group_ids_bbox_vec.specialize_policy().unwrap();
    let group_ids_bbox_vec: Vec<_> = group_ids_bbox_vec.into();

    let mut group_bbox_vec: Vec<StudentGroup> = Vec::new();
    for group_id in group_ids_bbox_vec {
        let group = StudentGroup {
            group_id: group_id.clone().into_bbox(),
            code: read_buffer(class_id_read_buffer_pol.clone(), group_id, context.clone()),
        };
        group_bbox_vec.push(group);
    }

    JsonResponse::from((
        InstructorResponse {
            success: true,
            class_id: class_id_bbox.into_bbox(),
            class_name: class_name_bbox.into_bbox(),
            students: students_bbox_vec,
            student_groups: group_bbox_vec,
        },
        context,
    ))
}

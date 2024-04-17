use crate::backend::MySqlBackend;
use crate::common::{Student, StudentGroup};
use crate::context::ContextDataType;
use crate::policies::VoltronBufferPolicy;
use alohomora::context::Context;
use alohomora::fold::fold;
use alohomora::pure::{execute_pure, PrivacyPureRegion};
use alohomora::rocket::{get, ContextResponse, JsonResponse, ResponseBBoxJson};
use alohomora::{
    bbox::BBox,
    db::from_value,
    policy::{AnyPolicy, NoPolicy},
};
use mysql::Value;
use rocket::serde::json::Json;
use rocket::State;
use rocket_firebase_auth::FirebaseToken;
use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::iter::FromIterator;
use std::sync::{Arc, Mutex};

#[derive(ResponseBBoxJson)]
pub struct InstructorResponse {
    pub success: bool,
    pub class_id: BBox<i64, VoltronBufferPolicy>,
    pub students: Vec<Student>,
    pub student_groups: Vec<StudentGroup>,
}

#[get("/instructor")]
pub(crate) fn instructor(
    token: BBox<FirebaseToken, NoPolicy>,
    backend: &State<Arc<Mutex<MySqlBackend>>>,
    context: Context<ContextDataType>,
) -> JsonResponse<InstructorResponse, ContextDataType> {
    print!("INSTRUCTOR\n");
    // Find this instructor
    let email_bbox: BBox<String, NoPolicy> =
        token.into_ppr(PrivacyPureRegion::new(|token: FirebaseToken| {
            // Need the following separate lines to give email a type
            let email: String = token.email.unwrap();
            email
        }));
    let mut bg: std::sync::MutexGuard<'_, MySqlBackend> = backend.lock().unwrap();
    // Get this instructor's class ID
    let user_res: Vec<Vec<BBox<Value, AnyPolicy>>> = (*bg).prep_exec(
        "SELECT * FROM users WHERE email = ?",
        vec![email_bbox.clone()],
        context.clone(),
    );
    // If the instructor is not found, return error
    if user_res.len() == 0 {
        /*
        let response: Json<InstructorResponse> = Json(InstructorResponse {
            success: false,
            class_id: -1,
            students: vec![],
            student_groups: vec![],
        });
        let response_bbox = BBox::new(response, AnyPolicy::new(NoPolicy {}));
        return ContextResponse::from((response_bbox, context));
        */
        panic!("bad user");
    }
    let row: Vec<BBox<Value, AnyPolicy>> = user_res[0].clone();
    let class_id_bbox: BBox<i32, VoltronBufferPolicy> = from_value(row[3].clone()).unwrap();

    let students_res: Vec<Vec<BBox<Value, AnyPolicy>>> = (*bg).prep_exec(
        "SELECT * FROM users WHERE privilege = 0 AND class_id = ?",
        vec![class_id_bbox.clone()], // hangs because of class_id_bbox
        context.clone(),
    );
    drop(bg);

    let mut group_ids_bbox_vec: Vec<BBox<i32, VoltronBufferPolicy>> = Vec::new();
    let mut students_bbox_vec: Vec<Student> = Vec::new();
    for row in students_res.iter() {
        let stud_name_bbox: BBox<String, NoPolicy> = from_value(row[0].clone()).unwrap();
        let group_id_bbox: BBox<i32, VoltronBufferPolicy> = from_value(row[3].clone()).unwrap();
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
    let group_ids_bbox_vec: BBox<_, VoltronBufferPolicy> =
        group_ids_bbox_vec.specialize_policy().unwrap();
    let group_ids_bbox_vec: Vec<_> = group_ids_bbox_vec.into();

    let mut group_bbox_vec: Vec<StudentGroup> = Vec::new();
    for group_id in group_ids_bbox_vec {
        let code = execute_pure(
            (class_id_bbox.clone(), group_id.clone()),
            PrivacyPureRegion::new(|(class_id, group_id)| {
                let filepath: String =
                    format!("../group_code/class{}_group{}_code.txt", class_id, group_id);
                fs::read_to_string(filepath).expect("Unable to read the file")
            }),
        )
        .unwrap();
        let group = StudentGroup {
            group_id: group_id.clone().into_bbox(),
            code: code.specialize_policy().unwrap(),
        };
        group_bbox_vec.push(group);
    }

    JsonResponse::from((
        InstructorResponse {
            success: true,
            class_id: class_id_bbox.into_bbox(),
            students: students_bbox_vec,
            student_groups: group_bbox_vec,
        },
        context,
    ))

    /*
    println!("Creating response:");
    use alohomora::policy::Policy;
    println!("{}", class_id_bbox.policy().name());
    println!("{}", group_ids_bbox_vec.policy().name());
    println!("{}", students_bbox_vec.policy().name());
    let response = execute_pure(
        (
            class_id_bbox.clone(),
            group_ids_bbox_vec.clone(),
            students_bbox_vec,
        ),
        PrivacyPureRegion::new(
            |(class_id, group_ids_og, students): (i32, Vec<i32>, Vec<Student>)| {
                println!("Entered the ppr");
                // Sort and remove duplicates from list of all group_ids in the class
                let mut group_ids = group_ids_og.clone();
                group_ids.sort();
                group_ids.dedup();
                let mut groups_res: Vec<StudentGroup> = Vec::new();
                // Read from the files to create StudentGroup vector
                for (index, group_id) in group_ids.iter().enumerate() {
                    let filepath: String =
                        format!("../group_code/class{}_group{}_code.txt", class_id, group_id);
                    let code: String =
                        fs::read_to_string(filepath).expect("Unable to read the file");
                    let stud_group: StudentGroup = StudentGroup {
                        group_id: *group_id,
                        code,
                        index,
                    };
                    groups_res.push(stud_group);
                }

                println!("About to return success json from the ppr");
                // Return response
                Json(InstructorResponse {
                    success: true,
                    class_id,
                    students,
                    student_groups: groups_res,
                })
            },
        ),
    )
    .unwrap();
    ContextResponse::from((response, context))
    */
}

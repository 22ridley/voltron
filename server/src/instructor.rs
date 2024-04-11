use std::fs;
use crate::common::{Student, StudentGroup, ApiResponse};
use crate::backend::MySqlBackend;
use alohomora::fold::fold;
use mysql::Value;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use serde::Serialize;
use std::sync::{Arc, Mutex};
use rocket_firebase_auth::FirebaseToken;
use alohomora::context::Context;
use crate::policies::ContextDataType;
use alohomora::rocket::get;
use alohomora::pure::{execute_pure, PrivacyPureRegion};
use alohomora::{bbox::BBox, db::from_value, policy::{AnyPolicy, NoPolicy}};

#[derive(Debug, Serialize)]
pub struct InstructorResponse {
    pub success: bool,
    pub class_id: i32,
    pub students: Vec<Student>,
    pub student_groups: Vec<StudentGroup>
}

#[get("/instructor")]
pub(crate) fn instructor(token: BBox<FirebaseToken, NoPolicy>, 
    backend: &State<Arc<Mutex<MySqlBackend>>>,
    context: Context<ContextDataType>) 
    -> ApiResponse<InstructorResponse> {
    // Find this instructor
    let email_bbox: BBox<String, AnyPolicy> = execute_pure(token, 
        PrivacyPureRegion::new(|token: FirebaseToken| {
            // Need the following separate lines to give email a type
            let email: String = token.email.unwrap();
            email
        })
    ).unwrap();
    let mut bg: std::sync::MutexGuard<'_, MySqlBackend> = backend.lock().unwrap();
    // Get this instructor's class ID
    let user_res: Vec<Vec<BBox<Value, AnyPolicy>>> = (*bg).prep_exec("SELECT * FROM users WHERE email = ?", vec![email_bbox.clone()], context);
    // If the instructor is not found, return error
    if user_res.len() == 0 {
        return ApiResponse {
            json: Some(Json(InstructorResponse {
                success: false,
                class_id: -1,
                students: vec![],
                student_groups: vec![]
            })),
            status: Status::InternalServerError,
        }
    }
    let row: Vec<BBox<Value, AnyPolicy>> = user_res[0];
    let class_id_bbox: BBox<i32, AnyPolicy> = from_value(row[3]).unwrap();
    let students_res: Vec<Vec<BBox<Value, AnyPolicy>>> = (*bg).prep_exec("SELECT * FROM users WHERE privilege = 0 AND class_id = ?", vec![class_id_bbox], context);
    let group_ids_row: Vec<Vec<BBox<Value, AnyPolicy>>> = (*bg).prep_exec("SELECT group_id FROM users WHERE class_id = ? AND group_id != -1", vec![class_id_bbox], context);
    let mut group_ids_bbox_vec: Vec<BBox<i32, AnyPolicy>> = Vec::new();
    let mut students_bbox_vec: Vec<BBox<Student, AnyPolicy>> = Vec::new();
    for row in group_ids_row.iter() {
        let group_id: BBox<i32, AnyPolicy> = from_value(row[0]).unwrap();
        group_ids_bbox_vec.push(group_id);
    }
    for row in students_res.iter() {
        let stud_name_bbox: BBox<String, AnyPolicy> = from_value(row[0]).unwrap();
        let group_id_bbox: BBox<i32, AnyPolicy> = from_value(row[3]).unwrap();
        let student_bbox: BBox<Student, AnyPolicy> = execute_pure((stud_name_bbox, group_id_bbox), 
            PrivacyPureRegion::new(|(name, group_id): (String, i32)| {
                Student{name, group_id, index: 0}
            })
        ).unwrap();
        students_bbox_vec.push(student_bbox);
    }
    // Moving BBox to outside
    let group_ids_bbox_vec: BBox<Vec<i32>, AnyPolicy> = fold(group_ids_bbox_vec).unwrap();
    let students_bbox_vec: BBox<Vec<Student>, AnyPolicy> = fold(students_bbox_vec).unwrap();

    let response = execute_pure((class_id_bbox, group_ids_bbox_vec, students_bbox_vec), 
        PrivacyPureRegion::new(|(class_id, group_ids, students): (i32, Vec<i32>, Vec<Student>)| {
            // Sort and remove duplicates from list of all group_ids in the class
            group_ids.sort();
            group_ids.dedup();
            let mut groups_res: Vec<StudentGroup> = Vec::new();
            // Read from the files to create StudentGroup vector
            for (index, group_id) in group_ids.iter().enumerate() {
                let filepath: String = format!("../group_code/class{}_group{}_code.txt", class_id, group_id);
                let code: String = fs::read_to_string(filepath).expect("Unable to read the file");
                let stud_group: StudentGroup = StudentGroup{group_id: *group_id, code, index: index};
                groups_res.push(stud_group);
            }
            drop(bg);

            // Return response
            ApiResponse {
                json: Some(Json(InstructorResponse {
                    success: true,
                    class_id: class_id,
                    students: students,
                    student_groups: groups_res
                })),
                status: Status::Ok,
            }
        })
    ).unwrap();
    response
}
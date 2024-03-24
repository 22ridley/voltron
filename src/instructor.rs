use std::fs;
use crate::common::{Student, StudentGroup, ApiResponse};
use crate::backend::MySQLBackend;
use mysql::Row;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{Route, State};
use serde::Serialize;
use std::sync::{Arc, Mutex};
use rocket_firebase_auth::FirebaseToken;

pub fn routes() -> Vec<Route> {
    routes![instructor]
}

#[derive(Debug, Serialize)]
pub struct InstructorResponse {
    pub success: bool,
    pub class_id: i32,
    pub group_id: i32,
    pub students: Vec<Student>,
    pub student_groups: Vec<StudentGroup>
}

#[get("/instructor")]
pub fn instructor(token: FirebaseToken, backend: &State<Arc<Mutex<MySQLBackend>>>) 
    -> ApiResponse<InstructorResponse> {
    let mut bg: std::sync::MutexGuard<'_, MySQLBackend> = backend.lock().unwrap();
    // Get this instructor's class ID
    let email: String = token.email.unwrap();
    let user_res: Vec<Row> = (*bg).prep_exec("SELECT * FROM users WHERE email = ?", vec![email.clone()]).unwrap();
    // If the instructor is not found, return error
    if user_res.len() == 0 {
        return ApiResponse {
            json: Some(Json(InstructorResponse {
                success: false,
                class_id: -1,
                group_id: -1,
                students: vec![],
                student_groups: vec![]
            })),
            status: Status::InternalServerError,
        }
    }
    let row: Row = user_res.get(0).unwrap().clone();
    let class_id: i32 = row.get(3).unwrap();
    // Get list of all students in this class
    let mut args: Vec<String> = Vec::new();
    args.push(class_id.to_string());
    let students_res: Vec<Student> = (*bg).prep_exec("SELECT * FROM users WHERE privilege = 0 AND class_id = ?", args).unwrap();
    let group_ids_row: Vec<Row> = (*bg).prep_exec("SELECT group_id FROM users WHERE class_id = ? AND group_id != -1", vec![class_id]).unwrap();
    let mut group_ids_vec: Vec<i32> = Vec::new();
    for row in group_ids_row.iter() {
        let group_id: i32 = row.get(0).unwrap();
        group_ids_vec.push(group_id);
    }
    group_ids_vec.sort();
    group_ids_vec.dedup();
    let mut groups_res: Vec<StudentGroup> = Vec::new();
    // Read from the files to create StudentGroup vector
    for (index, group_id) in group_ids_vec.iter().enumerate() {
        let filepath: String = format!("group_code/class{}_group{}_code.txt", class_id, group_id);
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
            group_id: -1,
            students: students_res,
            student_groups: groups_res
        })),
        status: Status::Ok,
    }
}
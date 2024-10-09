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
    pub class_name: String,
    pub students: Vec<Student>,
    pub student_groups: Vec<StudentGroup>
}

#[get("/instructor")]
pub fn instructor(token: FirebaseToken, backend: &State<Arc<Mutex<MySQLBackend>>>) 
    -> ApiResponse<InstructorResponse> {
    let mut bg: std::sync::MutexGuard<'_, MySQLBackend> = backend.lock().unwrap();
    // Get this instructor's class ID
    let email: String = token.email.unwrap();
    let user_res: Vec<Row> = (*bg).prep_exec("SELECT * FROM user INNER JOIN class ON user.user_id = class.instructor_id WHERE email = ?", vec![email.clone()]).unwrap();
    let row: Row = user_res.get(0).unwrap().clone();
    let class_id: i32 = row.get(4).unwrap();
    let class_name: String = row.get(5).unwrap();

    // Get list of all students in this class
    let students_res: Vec<Student> = (*bg).prep_exec("SELECT * FROM user INNER JOIN enroll ON user.user_id = enroll.student_id WHERE class_id = ?", vec![class_id.clone()]).unwrap();
    let mut group_ids_vec: Vec<i32> = Vec::new();
    for student in students_res.iter() {
        let group_id: i32 = student.group_id;
        group_ids_vec.push(group_id);
    }
    group_ids_vec.sort();
    group_ids_vec.dedup();
    let mut groups_res: Vec<StudentGroup> = Vec::new();

    // Read from the files to create StudentGroup vector
    for (index, group_id) in group_ids_vec.iter().enumerate() {
        let path = format!("../group_code/class{}_group{}_code.txt", class_id, group_id);
        let content_result = fs::read_to_string(path);
        let code: String = match content_result {
            // If this file does not exist, return empty string
            Err(_) => "".to_string(),
            // Otherwise, return the file content
            Ok(msg) => msg.to_string(),
        };
        let stud_group: StudentGroup = StudentGroup{group_id: *group_id, code, index};
        groups_res.push(stud_group);
    }
    drop(bg);

    // Return response
    ApiResponse {
        json: Some(Json(InstructorResponse {
            success: true,
            class_id: class_id,
            class_name: class_name,
            students: students_res,
            student_groups: groups_res
        })),
        status: Status::Ok,
    }
}
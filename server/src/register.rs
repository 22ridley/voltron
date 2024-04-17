extern crate mysql;
extern crate serde;
use crate::backend::MySqlBackend;
use crate::common::SuccessResponse;
use crate::context::ContextDataType;
use crate::policies::{InstructorPolicy, StudentPolicy};
use alohomora::context::Context;
use alohomora::rocket::post;
use alohomora::{bbox::BBox, policy::NoPolicy};
use rocket::serde::json::Json;
use rocket::State;
use rocket_firebase_auth::FirebaseToken;
use std::{sync::Arc, sync::Mutex};

#[post("/register_instructor?<instr_name>&<instr_class>&<instr_email>")]
pub fn register_instructor(
    _token: BBox<FirebaseToken, NoPolicy>,
    instr_name: BBox<String, InstructorPolicy>,
    instr_class: BBox<i32, InstructorPolicy>,
    instr_email: BBox<String, InstructorPolicy>,
    backend: &State<Arc<Mutex<MySqlBackend>>>,
    context: Context<ContextDataType>,
) -> Json<SuccessResponse> {
    // Make insert query to add this new instructor
    let mut bg = backend.lock().unwrap();

    // send insert query to db
    bg.prep_exec(
        "INSERT INTO users (user_name, email, privilege, class_id, group_id) VALUES (?, ?, ?, ?, ?)",
        (instr_name, instr_email, 1i32, instr_class, -1i32),
        context.clone(),
    );
    drop(bg);

    Json(SuccessResponse {
        success: true,
        message: "".to_string(),
    })
}

#[post("/register_student?<stud_group>&<stud_name>&<stud_class>&<stud_email>")]
pub fn register_student(
    _token: BBox<FirebaseToken, NoPolicy>,
    stud_group: BBox<i32, StudentPolicy>,
    stud_name: BBox<String, StudentPolicy>,
    stud_class: BBox<i32, StudentPolicy>,
    stud_email: BBox<String, StudentPolicy>,
    backend: &State<Arc<Mutex<MySqlBackend>>>,
    context: Context<ContextDataType>,
) -> Json<SuccessResponse> {
    let mut bg = backend.lock().unwrap();
    // Make insert query to add this new student into users
    let users_row = (
        stud_name,
        stud_email,
        0i32,
        stud_class.clone(),
        stud_group.clone(),
    );
    let q: &str = "INSERT INTO users (user_name, email, privilege, class_id, group_id) VALUES (?, ?, ?, ?, ?)";
    (*bg).prep_exec(q, users_row, context.clone());
    drop(bg);

    // let path: BBox<String, StudentPolicy> = execute_pure(
    //     (stud_group, stud_class),
    //     PrivacyPureRegion::new(|(s_group, s_class): (i32, i32)| {
    //         format!("../group_code/class{}_group{}_code.txt", s_class, s_group)
    //     }),
    // )
    // .unwrap()
    // .specialize_policy()
    // .unwrap();

    // Opening a new file
    // Needs to be a privacy critical region, or moved into other pcr
    // path.unbox(
    //     context.clone(),
    //     PrivacyCriticalRegion::new(|path, ()| {
    //         let file_name: &Path = Path::new(&path);
    //         if !file_name.is_file() {
    //             // Open a new file
    //             let _ = File::create(file_name);
    //         }
    //     }),
    //     (),
    // )
    // .unwrap();

    return Json(SuccessResponse {
        success: true,
        message: "".to_string(),
    });
}

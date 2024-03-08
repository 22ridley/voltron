use std::{cmp, fs};
use crate::common::{AnyResponse, InstructorContext, Student, StudentGroup};
use crate::backend::MySQLBackend;
use rocket::State;
use rocket_dyn_templates::Template;
use std::sync::{Arc, Mutex};

#[get("/?<name>&<reg_name>&<reg_type>")]
pub fn instructor(name: &str, reg_name: Option<&str>, reg_type: Option<&str>, 
    backend: &State<Arc<Mutex<MySQLBackend>>>) -> AnyResponse {
    // Get the user information from the backend database
    let mut is_admin: bool = false;
    let mut register_name: &str = "";
    let mut reg_instructor: bool = false;
    let mut reg_student: bool = false;
    if name == "admin" {
        is_admin = true;
    }
    if reg_name.is_some() & reg_type.is_some() {
        let register_type: &str = reg_type.unwrap();
        if register_type == "inst" {
            reg_instructor = true
        } else if register_type == "stud" {
            reg_student = true
        }
        register_name = reg_name.unwrap();
    }

    // Get list of all students
    let mut bg: std::sync::MutexGuard<'_, MySQLBackend> = backend.lock().unwrap();
    let students_res: Vec<Student> = (*bg).prep_exec("SELECT * FROM users WHERE privilege = 0", ()).unwrap();
    let mut max_student_group: i32 = 0;
    for student in students_res.iter() {
        let group_id: i32 = student.group_id;
        max_student_group = cmp::max(max_student_group, group_id);
    }
    let group_numbers: Vec<i32> = (0..max_student_group+1).collect::<Vec<i32>>();
    let mut groups_res: Vec<StudentGroup> = Vec::new();
    // Read from the files to create StudentGroup vector
    for (index, id) in group_numbers.iter().enumerate() {
        let filepath: String = format!("group_code/group{}_code.txt", id);
        let code: String = fs::read_to_string(filepath).expect("Unable to read the file");
        let stud_group: StudentGroup = StudentGroup{group_id: *id, code, index: index};
        groups_res.push(stud_group);
    }
    drop(bg);

    // Create the context for the template
    let ctx: InstructorContext = InstructorContext {
        name: name.to_string(),
        admin: is_admin,
        registered_name: register_name.to_string(),
        registered_instructor: reg_instructor,
        registered_student: reg_student,
        students: students_res,
        student_groups: groups_res
    };
    AnyResponse::Template(Template::render("instructor", &ctx))
}
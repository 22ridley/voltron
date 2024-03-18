use std::{cmp, fs};
use crate::common::{AnyResponse, InstructorContext, Student, StudentGroup};
use crate::backend::MySQLBackend;
use rocket::State;
use rocket_dyn_templates::Template;
use std::sync::{Arc, Mutex};

#[get("/?<name>&<class_id>&<reg_name>")]
pub fn instructor(name: &str, class_id: i32, reg_name: Option<&str>, 
    backend: &State<Arc<Mutex<MySQLBackend>>>) -> AnyResponse {
    // Get the user information from the backend database
    let mut register_name: &str = "";
    let mut reg = false;
    if reg_name.is_some()  {
        register_name = reg_name.unwrap();
        reg = true;
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
        class_id: class_id,
        registered_name: register_name.to_string(),
        registered_student: reg,
        students: students_res,
        student_groups: groups_res
    };
    AnyResponse::Template(Template::render("instructor", &ctx))
}
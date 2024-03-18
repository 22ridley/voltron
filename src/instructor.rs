use std::fs;
use crate::common::{AnyResponse, InstructorContext, Student, StudentGroup};
use crate::backend::MySQLBackend;
use mysql::Row;
use rocket::State;
use rocket_dyn_templates::Template;
use std::sync::{Arc, Mutex};

#[get("/?<name>&<class_id>&<reg_name>")]
pub fn instructor(name: &str, class_id: &str, reg_name: Option<&str>, 
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

    // Create the context for the template
    let ctx: InstructorContext = InstructorContext {
        name: name.to_string(),
        class_id: class_id.to_string(),
        registered_name: register_name.to_string(),
        registered_student: reg,
        students: students_res,
        student_groups: groups_res
    };
    AnyResponse::Template(Template::render("instructor", &ctx))
}
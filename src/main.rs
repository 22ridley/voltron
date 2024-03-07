extern crate serde;
extern crate mysql;
#[macro_use]
extern crate rocket;
extern crate rocket_dyn_templates;
use std::io::Write;
use std::{sync::Arc, sync::Mutex, cmp};
use common::{InstructorContext, StudentContext, Student, StudentGroup, AnyResponse, LoginContext};
use mysql::{prelude::Queryable, Row};
use backend::MySQLBackend;
use rocket::{response::Redirect, State};
use rocket_dyn_templates::Template;
use std::path::Path;
use std::fs::{self, File};

mod config;
mod common;
mod backend;
mod register;

#[rocket::main]
async fn main() {
    // Get the config file
    let config_path = "config.toml";
    let config = config::parse(config_path).unwrap();

    // Make the template
    let template_dir: String = config.template_dir.clone();
    let template = Template::custom(move |engines| {
        engines
            .handlebars
            .register_templates_directory(".hbs", std::path::Path::new(&template_dir))
            .expect("failed to set template path!");
    });

    // Initialize the backend
    let db_name: &str = "users";
    let backend: Arc<Mutex<MySQLBackend>> = Arc::new(Mutex::new(
        backend::MySQLBackend::new(
            &config.db_user,
            &config.db_password,
            &format!("{}", db_name),
            config.prime
        ).unwrap()
    ));

    // build and launch
    if let Err(e) = rocket::build()
        .attach(template)
        .manage(backend)
        .manage(config)
        .mount("/", routes![index])
        .mount("/login", routes![login])
        .mount("/view", routes![view])
        .mount("/instructor", routes![instructor])
        .mount("/student", routes![student])
        .mount("/register-instructor", routes![register::register_instructor])
        .mount("/register-student", routes![register::register_student])
        .launch()
        .await 
    {
        println!("Didn't launch properly");
        drop(e);
    };
}

// Index redirects to login
#[get("/")]
fn index() -> Redirect {
    Redirect::to("/login")
}

// Login page directs instructor to class view
// Login page directs students to student view
// Grouped students see the same student view
#[get("/?<fail>")]
pub fn login(fail: Option<&str>) -> Template {
    let mut retry: bool = false;
    if fail.is_some() {
        retry = true;
    }
    let ctx: LoginContext = LoginContext {
        failed: retry
    };
    Template::render("login", ctx)
}

#[get("/?<name>")]
pub fn view(name: &str, backend: &State<Arc<Mutex<MySQLBackend>>>) 
 -> AnyResponse {
    let mut bg: std::sync::MutexGuard<'_, MySQLBackend> = backend.lock().unwrap();
    let user_res: Vec<Row> = (*bg).handle.query(format!("SELECT * FROM users WHERE user_name = \"{}\"", name)).unwrap();
    drop(bg);
    if user_res.len() == 0 { return AnyResponse::Redirect(Redirect::to("/login?fail")); }
    let row: Row = user_res.get(0).unwrap().clone();
    let privilege: Option<i32> =  row.get(1).unwrap();
    let group_id: Option<i32> = row.get(2).unwrap();
    if privilege.unwrap() != 0 {
        AnyResponse::Redirect(Redirect::to(format!("/instructor?name={}", name)))
    } else {
        AnyResponse::Redirect(Redirect::to(format!("/student?name={}&group_id={}", name, group_id.unwrap())))
    }
}

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
    let students_res: Vec<Student> = (*bg).handle.query(format!("SELECT * FROM users WHERE privilege = 0")).unwrap();
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

#[get("/?<name>&<group_id>&<text>")]
pub fn student(name: &str, group_id: &str, text: Option<&str>) -> AnyResponse {
    // File path to read and write from
    let filepath: String = format!("group_code/group{}_code.txt", group_id);
    // First, see if there is submitted text
    // If there is text, write it to the database and redirect to the same page
    //    but without the text to make the url cleaner :)
    if text.is_some() {
        // Open a file in write-only mode, returns `io::Result<File>`
        let path: &Path = Path::new(&filepath);
        let mut file: File = File::create(&path).unwrap();

        // Write the new text to the file
        let _bytes_written: Result<usize, std::io::Error> = file.write(text.unwrap().as_bytes());
        return AnyResponse::Redirect(Redirect::to(format!("/student?name={}&group_id={}", name, group_id)));
    }
    
    // Convert group_id to number
    let group_id_num: i32 = group_id.parse().unwrap();
    let contents: String = fs::read_to_string(filepath).expect("Unable to read file");

    let ctx: StudentContext = StudentContext {
        name: name.to_string(),
        group_id: group_id_num,
        text: contents
    };
    AnyResponse::Template(Template::render("student", &ctx))
}
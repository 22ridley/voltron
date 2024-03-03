extern crate serde;
extern crate mysql;
#[macro_use]
extern crate rocket;
extern crate rocket_dyn_templates;
use std::{sync::Arc, sync::Mutex, cmp};
use common::{InstructorContext, Student, AnyResponse};
use mysql::{prelude::Queryable, Row};
use backend::MySQLBackend;
use rocket::{response::Redirect, State};
use rocket_dyn_templates::Template;

mod config;
mod common;
mod backend;

#[rocket::main]
async fn main() {
    // Get the config file
    let config_path = "config.toml";
    let config = config::parse(config_path).unwrap();

    // Make the template
    let template_dir = config.template_dir.clone();
    let template = Template::custom(move |engines| {
        engines
            .handlebars
            .register_templates_directory(".hbs", std::path::Path::new(&template_dir))
            .expect("failed to set template path!");
    });

    // Initialize the backend
    let db_name = "users";
    let backend = Arc::new(Mutex::new(
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
        .mount("/register-instructor", routes![register_instructor])
        .mount("/register-student", routes![register_student])
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
    let ctx = common::LoginContext {
        failed: retry
    };
    Template::render("login", ctx)
}

#[get("/?<name>")]
pub fn view(name: &str, backend: &State<Arc<Mutex<MySQLBackend>>>) 
 -> AnyResponse {
    let mut bg = backend.lock().unwrap();
    let user_res: Vec<Row> = (*bg).handle.query(format!("SELECT * FROM users WHERE user_name = \"{}\"", name)).unwrap();
    drop(bg);
    if user_res.len() == 0 { return AnyResponse::Redirect(Redirect::to("/login?fail")); }
    let row: Row = user_res.get(0).unwrap().clone();
    let row_name: Option<i32> =  row.get(1).unwrap();
    if row_name.unwrap() != 0 {
        AnyResponse::Redirect(Redirect::to(format!("/instructor?name={}", name)))
    } else {
        AnyResponse::Redirect(Redirect::to(format!("/student/{}", name)))
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
    let mut bg = backend.lock().unwrap();
    let students_res: Vec<Student> = (*bg).handle.query(format!("SELECT * FROM users WHERE privilege = 0")).unwrap();

    // Create the context for the template
    let ctx: InstructorContext = InstructorContext {
        name: name.to_string(),
        admin: is_admin,
        registered_name: register_name.to_string(),
        registered_instructor: reg_instructor,
        registered_student: reg_student,
        students: students_res
    };
    AnyResponse::Template(Template::render("instructor", &ctx))
}

#[get("/<name>")]
pub fn student(name: &str) -> Template {
    let ctx = common::StudentContext {
        name: name.to_string()
    };
    Template::render("student", &ctx)
}

#[get("/?<name>")]
pub fn register_instructor(name: &str, backend: &State<Arc<Mutex<MySQLBackend>>>) 
-> AnyResponse {
    // Assemble values to insert
    let users_row: Vec<&str> = vec![name, "1", "-1"];

    // Make insert query to add this new instructor
    let q = format!("INSERT INTO users (user_name, privilege, group_id) VALUES ({})", 
                            users_row.iter().map(|s| {format!("\"{s}\"")})
                                .collect::<Vec<String>>()
                                .join(","));

    // send insert query to db
    let mut bg = backend.lock().unwrap();
    let _ = (*bg).handle.query_drop(q).unwrap();
    drop(bg);

    AnyResponse::Redirect(Redirect::to(format!("/instructor?name=admin&reg_name={}&reg_type={}", name, "inst")))
}

#[get("/?<name>&<student_name>")]
pub fn register_student(name: &str, student_name: &str, 
    backend: &State<Arc<Mutex<MySQLBackend>>>)-> AnyResponse {
    // Count the number of students currently in the database 
    let mut bg = backend.lock().unwrap();
    let students_res: Vec<Row> = (*bg).handle.query(format!("SELECT * FROM users WHERE privilege = 0")).unwrap();    
    let mut max_student_group: i32 = 0;
    for student in students_res.iter() {
        let group_id: i32 = student.clone().get(2).unwrap();
        max_student_group = cmp::max(max_student_group, group_id);
    }
    let mut student_group: i32 = max_student_group;
    // There are an even number of students, so there are no students alone
    if students_res.len() % 2 == 0 {
        // The student will have group_id max_student_group + 1
        student_group += 1;
    }
    // Otherwise, there are an odd number of students, so there is some student alone
    // and the student will have group_id max_student_group
    let student_group_string: &str = &student_group.to_string();
    let users_row: Vec<&str> = vec![student_name, "0", student_group_string];    

    // Make insert query to add this new student into users
    let q = format!("INSERT INTO users (user_name, privilege, group_id) VALUES ({})", 
                            users_row.iter().map(|s| {format!("\"{s}\"")})
                                .collect::<Vec<String>>()
                                .join(","));
    let _ = (*bg).handle.query_drop(q).unwrap();

    // Make insert query to add a new student_group to student_groups (if necessary)
    if students_res.len() % 2 == 0 {
        let student_group_row: Vec<&str> = vec![student_group_string, ""];  
        let q = format!("INSERT INTO student_groups (group_id, code) VALUES ({})", 
                            student_group_row.iter().map(|s| {format!("\"{s}\"")})
                                .collect::<Vec<String>>()
                                .join(","));
        let _ = (*bg).handle.query_drop(q).unwrap();
    }
    drop(bg);
    AnyResponse::Redirect(Redirect::to(format!("/instructor?name={}&reg_name={}&reg_type={}", name, student_name, "stud")))
}
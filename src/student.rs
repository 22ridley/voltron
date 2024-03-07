use std::{io::Write, path::Path};
use crate::common::{UpdateRequest, StudentContext, AnyResponse};
use rocket::{response::Redirect, form::Form};
use std::fs::{self, File};
use rocket_dyn_templates::Template;

#[get("/?<name>&<group_id>")]
pub fn student(name: &str, group_id: &str) -> AnyResponse {
    // File path to read and write from
    let filepath: String = format!("group_code/group{}_code.txt", group_id);
    
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

#[post("/", data="<data>")]
pub fn update(data: Form<UpdateRequest>) -> AnyResponse {
    // Open a file in write-only mode, returns `io::Result<File>`
    let filepath: String = format!("group_code/group{}_code.txt", data.group_id);
    let path: &Path = Path::new(&filepath);
    let mut file: File = File::create(&path).unwrap();

    // Write the new text to the file
    let _bytes_written: Result<usize, std::io::Error> = file.write(data.text.as_bytes());
    return AnyResponse::Redirect(Redirect::to(format!("/student?name={}&group_id={}", data.name, data.group_id)));
}


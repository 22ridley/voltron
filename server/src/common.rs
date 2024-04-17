use alohomora::bbox::BBox;
use alohomora::context::Context;
use alohomora::pcr::PrivacyCriticalRegion;
use alohomora::policy::{NoPolicy, Policy};
use alohomora::rocket::ResponseBBoxJson;
use alohomora::unbox::unbox;
use mysql::from_value;
use mysql::prelude::FromRow;
use mysql::serde::Serialize;
use mysql::Row;
use rocket::http::{ContentType, Status};
use rocket::response;
use rocket::serde::json::Json;
use rocket::{Request, Response};
use std::fs::{self, File};

use crate::context::ContextDataType;
use crate::policies::{VoltronBufferPolicy, WriteBufferPolicy};

/// The struct we return for success responses (200s)
#[derive(Debug)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    pub json: Option<Json<T>>,
    pub status: Status,
}

#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    pub success: bool,
    pub message: String,
}

/// Implements the `Responder` trait for Rocket, so we can simply return a for
/// endpoint functions, result and Rocket takes care of the rest.
impl<'r, T: Serialize> response::Responder<'r, 'r> for ApiResponse<T> {
    fn respond_to(self, req: &'r Request) -> response::Result<'r> {
        Response::build_from(self.json.respond_to(req)?)
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

// The context needed for rendering the login page
#[derive(Serialize)]
pub struct LoginContext {
    pub failed: bool,
}

// The structure representing student groups and their code
#[derive(ResponseBBoxJson)]
pub struct StudentGroup {
    pub group_id: BBox<i64, VoltronBufferPolicy>,
    pub code: BBox<String, VoltronBufferPolicy>,
}

// The structure representing instructors
#[derive(Debug, Serialize)]
pub struct Instructor {
    pub name: String,
    pub class_id: i32,
    pub index: usize,
}

impl Instructor {
    pub fn new(row: Row, index: usize) -> Self {
        Instructor {
            name: from_value(row[0].clone()),
            class_id: from_value(row[3].clone()),
            index: index,
        }
    }
}

impl FromRow for Instructor {
    fn from_row_opt(row: Row) -> Result<Self, mysql::FromRowError> {
        Ok(Instructor::new(row, 0))
    }

    fn from_row(row: Row) -> Self {
        Instructor::new(row, 0)
    }
}

// The structure representing students
use std::collections::HashMap;
#[derive(ResponseBBoxJson)]
pub struct Student {
    pub name: BBox<String, NoPolicy>,
    pub group_id: BBox<i64, VoltronBufferPolicy>,
}

pub fn read_buffer<P: Policy + Clone + 'static>(
    class_id: BBox<i32, P>,
    group_id: BBox<i32, P>,
    context: Context<ContextDataType>,
) -> BBox<String, VoltronBufferPolicy> {
    println!("{}", class_id.policy().name());
    println!("{}", group_id.policy().name());
    unbox(
        (class_id, group_id),
        context,
        PrivacyCriticalRegion::new(|(class_id, group_id): (i32, i32), ()| {
            println!("Inside pcr");
            let path = format!("../group_code/class{}_group{}_code.txt", class_id, group_id);
            let content_result = fs::read_to_string(path);
            let content: String = match content_result {
                // If this file does not exist, return empty string
                Err(_) => "".to_string(),
                // Otherwise, return the file content
                Ok(msg) => msg.to_string(),
            };
            println!("Content: {}", content);
            BBox::new(content, VoltronBufferPolicy::new(class_id, group_id))
        }),
        (),
    )
    .unwrap()
}

use std::io::Write;
pub fn write_buffer<P: Policy + Clone + 'static>(
    class_id: BBox<i32, P>,
    group_id: BBox<i32, P>,
    context: Context<ContextDataType>,
    contents: BBox<String, WriteBufferPolicy>,
) {
    contents
        .into_unbox(
            context,
            PrivacyCriticalRegion::new(|contents: String, (class_id, group_id): (i32, i32)| {
                let path = format!("../group_code/class{}_group{}_code.txt", class_id, group_id);
                let mut file: File = File::create(&path).unwrap();
                let _bytes_written: Result<usize, std::io::Error> = file.write(contents.as_bytes());
            }),
            (class_id, group_id),
        )
        .unwrap();
}

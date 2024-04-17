use alohomora::bbox::BBox;
use alohomora::policy::NoPolicy;
use alohomora::rocket::ResponseBBoxJson;
use mysql::from_value;
use mysql::prelude::FromRow;
use mysql::serde::Serialize;
use mysql::Row;
use rocket::http::{ContentType, Status};
use rocket::response;
use rocket::serde::json::Json;
use rocket::{Request, Response};

use crate::policies::VoltronBufferPolicy;

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

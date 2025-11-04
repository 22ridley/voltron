use mysql::serde::Serialize;
use rocket::http::{ContentType, Status};
use rocket::response;
use rocket::serde::json::Json;
use rocket::{Request, Response};
use rocket_firebase_auth::FirebaseToken;
use std::collections::HashMap;
use std::fs::{self, File};

use sesame::context::Context;
use sesame::critical::{execute_critical, CriticalRegion, Signature};
use sesame::pcon::PCon;
use sesame::policy::{AnyPolicy, Policy};
use sesame::verified::VerifiedRegion;
use sesame_rocket::rocket::ResponsePConJson;
use std::io::Write;

use crate::context::ContextDataType;
use crate::policies::{AuthStatePolicy, ReadBufferPolicy, WriteBufferPolicy};

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
#[derive(ResponsePConJson)]
pub struct StudentGroup {
    pub group_id: PCon<i64, ReadBufferPolicy>,
    pub code: PCon<String, ReadBufferPolicy>,
}

// The structure representing instructors
#[derive(ResponsePConJson)]
pub struct Instructor {
    pub name: PCon<String, AnyPolicy>,
    pub class_id: PCon<i32, AnyPolicy>,
}

// The structure representing students
#[derive(ResponsePConJson)]
pub struct Student {
    pub name: PCon<String, AnyPolicy>,
    pub group_id: PCon<i64, ReadBufferPolicy>,
}

pub fn email_from_token(
    token: PCon<FirebaseToken, AuthStatePolicy>,
) -> PCon<String, AuthStatePolicy> {
    token.into_verified(VerifiedRegion::new(|token: FirebaseToken| {
        let email: String = token.email.unwrap();
        email
    }))
}

pub fn read_buffer<P: Policy + Clone + 'static>(
    class_id: PCon<i32, P>,
    group_id: PCon<i32, P>,
    context: Context<ContextDataType>,
) -> PCon<String, ReadBufferPolicy> {
    execute_critical(
        (class_id, group_id),
        context,
        CriticalRegion::new(
            |(class_id, group_id): (i32, i32), ()| {
                let path = format!("../group_code/class{}_group{}_code.txt", class_id, group_id);
                let content_result = fs::read_to_string(path);
                let content: String = match content_result {
                    // If this file does not exist, return empty string
                    Err(_) => "".to_string(),
                    // Otherwise, return the file content
                    Ok(msg) => msg.to_string(),
                };
                PCon::new(content, ReadBufferPolicy::new(class_id, group_id))
            },
            Signature {
              username: "corinnt",
              signature: "LS0tLS1CRUdJTiBTU0ggU0lHTkFUVVJFLS0tLS0KVTFOSVUwbEhBQUFBQVFBQUFETUFBQUFMYzNOb0xXVmtNalUxTVRrQUFBQWd6dGJjeE9zVzlOL09Fd2c3Y3BKZ3dUQnFMNgpGazI2ZVB2Rm1ZaXpRRjM1VUFBQUFFWm1sc1pRQUFBQUFBQUFBR2MyaGhOVEV5QUFBQVV3QUFBQXR6YzJndFpXUXlOVFV4Ck9RQUFBRURmUkt0dU50eWZadUpYenJmcjdqOFd2aGxzTUI0R01RVHBJKzZpazM0T3RHVU1ybEU5ZHVJTUl4SEdGVHkyaHMKWVloTmNTMEFkR3pveXJETndIOGNZQQotLS0tLUVORCBTU0ggU0lHTkFUVVJFLS0tLS0K"
            },
        ),
        (),
    )
    .unwrap()
}

pub fn write_buffer<P: Policy + Clone + 'static>(
    class_id: PCon<i32, P>,
    group_id: PCon<i32, P>,
    context: Context<ContextDataType>,
    contents: PCon<String, WriteBufferPolicy>,
) {
    contents.into_critical(
        context,
        CriticalRegion::new(
            |contents: String, (class_id, group_id): (i32, i32)| {
                let path = format!("../group_code/class{}_group{}_code.txt", class_id, group_id);
                let mut file: File = File::create(&path).unwrap();
                let _bytes_written: Result<usize, std::io::Error> = file.write(contents.as_bytes());
            },
            Signature {
              username: "corinnt",
              signature: "LS0tLS1CRUdJTiBTU0ggU0lHTkFUVVJFLS0tLS0KVTFOSVUwbEhBQUFBQVFBQUFETUFBQUFMYzNOb0xXVmtNalUxTVRrQUFBQWd6dGJjeE9zVzlOL09Fd2c3Y3BKZ3dUQnFMNgpGazI2ZVB2Rm1ZaXpRRjM1VUFBQUFFWm1sc1pRQUFBQUFBQUFBR2MyaGhOVEV5QUFBQVV3QUFBQXR6YzJndFpXUXlOVFV4Ck9RQUFBRUI5c2w3cWRCQWNaSVNBYkQya3czWWV2ZDhVT1l5MVlpNFBqZFp6R1Qwd1RQQU91Wk9uL2lXTGJxUnRxeVozanoKemcxdlRTdmRyWlNpWE5SWHRTU2dVRgotLS0tLUVORCBTU0ggU0lHTkFUVVJFLS0tLS0K"
            },
        ),
        (class_id, group_id),
    )
    .unwrap();
}

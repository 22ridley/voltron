use rocket_firebase_auth::jwk::Jwk;
use rocket::serde::{Deserialize, Serialize};
use serde_json::json;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

pub static TEST_JWKS_URL: &str = "http://localhost:8888/jwks_url";
pub static JWK_N: &str = "XiqTPp559nQUm1dxEecu1HT_yD0Vv4OY9uVG-YNCy8NU2ceaqoUfF05luJ9xtmf-D3PQjpwSouA2ejxr_GSD2VQ99our7CDe5t5jdNEXyXSrXJDua5-lEETT-fOmnAuhti9m86w7Jd2HafJCq9sGEuQc4-CM1-2faerEGGl9iyNGQEZBnS_ExDJOJnbI8J6JZnWhL_rjwkVcM6_MX9bDA1Hi4fosMW1V-WhnZUJ5-WvNfX_2feXfQ0qq5kH9BWhbYCxwwT2d_0xkdL0aItEB_nqZSS9yIDiGQATjf_P708dEo9A0tVZg9HUtLY5k4-JCrOUayJOYJ5b-YGETzSbELw";
pub static KID: &str = "test_kid";

pub static INSTR_TOKEN: &str = "eyJhbGciOiJSUzI1NiIsImtpZCI6InRlc3Rfa2lkIn0.eyJzdWIiOiJzb21lLXVpZCIsImF1ZCI6ImR1bW15LXByb2plY3QtaWQiLCJpYXQiOjE2NjcwOTc2MjMsImV4cCI6NTAwNzA5OTYyMywiZW1haWwiOiJzYXJhaF9yaWRsZXlAYnJvd24uZWR1IiwiaXNzIjoiaHR0cHM6Ly9zZWN1cmV0b2tlbi5nb29nbGUuY29tL2R1bW15LXByb2plY3QtaWQifQ.RCRV9iT6245UMQEJ8App7rkMHqicLDkPC4ENgy3wxPECLRltpNfA6PADUZ_krxqcKto93thJjjAtujIAj7BXIhADOt2WPGCWcG1pC1zIZxNZ3mPBNQ62WuZNn7aoUAOxU6YuFHLXQP99goZmyw3fAX5N2PP5TWKfl8XoVAL2bl_s8WG5ogqJCCVfx3zM4fzJSbGuIphL51r8DjF_mPmfBs-EoBC1N8z8xZXKyVn0gR0aZM_lWb2ClJ-RNvW5yNdvlr_SwjUEXW6iZ1UL5tq-C-_Vae0hNzTaey-Uu98dEQk6STOR23ZZWtEwCzIr6hCUn1xkKpvdgmwPSP7JNqsllg";
pub static STUD_TOKEN: &str = "eyJhbGciOiJSUzI1NiIsImtpZCI6InRlc3Rfa2lkIn0.eyJzdWIiOiJzb21lLXVpZCIsImF1ZCI6ImR1bW15LXByb2plY3QtaWQiLCJpYXQiOjE2NjcwOTc2MjMsImV4cCI6NTAwNzA5OTYyMywiZW1haWwiOiJzYXJhaC5rYXRlLnJpZGxleUBnbWFpbC5jb20iLCJpc3MiOiJodHRwczovL3NlY3VyZXRva2VuLmdvb2dsZS5jb20vZHVtbXktcHJvamVjdC1pZCJ9.Sos5_qeTSxsCdotSOHdsldlTc5Y8WEZGcPl1ZTsM423NTbBE-DjF3jO6iveyetbYQSylwQ3toAXmkEvjU0zmLek7kDSvsdhKIpu8ccqv92YpH3sk1fdVspitdwnkxdQ5IYRy_Yoj0OTVPvtwd6_JrUiCVuTbMAsjvYzYOTT3oC9NNkYVmSOYMnh3UmPGggfLSHgJN2d53HjSCmrCOjV-Sqmua6Zr4wh8btDWkM19vVL1yYnjlNu9OLbjkCtVbc-Em0DM6LefgFIp0qxmWBJMwIK1FlqDv5uHZ0eeGY9HWYeREys-owBiwP1OyJmWIPrSZ0UpOlrwx2kSJC5ragTwmA";
pub static ADMIN_TOKEN: &str = "eyJhbGciOiJSUzI1NiIsImtpZCI6InRlc3Rfa2lkIn0.eyJzdWIiOiJzb21lLXVpZCIsImF1ZCI6ImR1bW15LXByb2plY3QtaWQiLCJpYXQiOjE2NjcwOTc2MjMsImV4cCI6NTAwNzA5OTYyMywiZW1haWwiOiIyMnJpZGxleXNrQGdtYWlsLmNvbSIsImlzcyI6Imh0dHBzOi8vc2VjdXJldG9rZW4uZ29vZ2xlLmNvbS9kdW1teS1wcm9qZWN0LWlkIn0.K-MAV5ds_rviiIB3AMENhyzBaacSl45PiOP6UWQk6v9A6x7bbRv3H7rqA5NxDYJBN_sOZfYZDlucaQfcNC_tDvxkwmq_Q86VEl-sLlFGBlkumjAg_dL7NMpYcq-8PmTDR6PBwpjHyRcj1uXHQvop8LGkDGaWoUAfzHXli0fPH0jBo5x4FfLxmtDK7m7rBStXz1wGaxNv7Gx8kVF51zsKe9bJDMoF7RBdPW7iiaQWqyR7vd3L1u9PI9U1al7rzB89LMQIt4hxYs8Dc5VqqcVp7LVS2xZ3DXFi6H1GdFfw1A6ONEfG9xv-uLqXThIKwQ6UWUATjlSU9ndn1qTMrgBWzQ";

#[derive(Deserialize)]
pub struct LoginResponse {
    pub email: String,
    pub name: String,
    pub privilege: i32,
    pub success: bool
}

#[derive(Deserialize)]
pub struct InstructorResponse {
    pub class_id: i32,
    pub student_groups: Vec<StudentGroup>,
    pub students: Vec<Student>,
    pub success: bool
}

#[derive(Deserialize)]
pub struct Student {
    pub name: String,
    pub group_id: i32
}

#[derive(Deserialize)]
pub struct StudentGroup {
    pub code: String,
    pub group_id: i64
}

#[derive(Deserialize)]
pub struct SuccessResponse {
    pub success: bool,
    pub message: String
}

#[derive(Deserialize)]
pub struct StudentResponse {
    pub success: bool,
    pub class_id: i64,
    pub group_id: i64,
    pub contents: Option<String>
}

#[derive(Deserialize)]
pub struct AdminResponse {
    pub success: bool,
    pub instructors: Vec<Instructor>
}

#[derive(Deserialize)]
pub struct Instructor {
    pub name: String,
    pub class_id: i32
}

pub async fn setup_mock_server() -> MockServer {
    let listener = std::net::TcpListener::bind("localhost:8888").unwrap();
    MockServer::builder().listener(listener).start().await
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MockJwksResponse {
    pub keys: Vec<Jwk>,
}

pub fn mock_jwk_issuer(jwks: &[Jwk]) -> Mock {
    Mock::given(method("GET"))
        .and(path("/jwks_url"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!(MockJwksResponse {
                keys: jwks.to_vec()
            })),
        )
}
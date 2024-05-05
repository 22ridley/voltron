use alohomora::testing::BBoxClient;
use rocket::http::{Header, Status};
use std::thread;
use voltron::build_server_test;
mod common;
use common::{JWK_N, KID, INSTR_TOKEN, STUD_TOKEN, ADMIN_TOKEN, setup_mock_server, mock_jwk_issuer};
use rocket_firebase_auth::jwk::Jwk;
use rocket::serde::Deserialize;

#[derive(Deserialize)]
struct LoginResponse {
    email: String,
    name: String,
    privilege: i32,
    success: bool
}

#[derive(Deserialize)]
struct InstructorResponse {
    class_id: i32,
    student_groups: Vec<StudentGroup>,
    students: Vec<Student>,
    success: bool
}

#[derive(Deserialize)]
struct Student {
    name: String,
    group_id: i32
}

#[derive(Deserialize)]
struct StudentGroup {
    pub code: String,
    pub group_id: i64
}

#[derive(Deserialize)]
struct SuccessResponse {
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

// RUN WITH RUST_TEST_THREADS=1 cargo test

// #[tokio::test]
// async fn test_login() {
//     let mock_jwk_server = setup_mock_server().await;
//     let jwk = Jwk::new(KID, JWK_N);

//     mock_jwk_issuer(vec![jwk].as_slice())
//         .mount(&mock_jwk_server)
//         .await;

//     let firebase_auth = FirebaseAuth::builder()
//         .json_file("./tests/dummy-firebase-creds.json")
//         .jwks_url(TEST_JWKS_URL)
//         .build()
//         .unwrap();
//     let decoded_token = firebase_auth.verify(INSTR_TOKEN).await;

//     assert!(decoded_token.is_ok());

//     let decoded_token = decoded_token.unwrap();

//     assert_eq!(decoded_token.sub, "some-uid");
//     assert_eq!(decoded_token.email, Some("sarah_ridley@brown.edu".to_string()));
//     assert!(decoded_token.exp > decoded_token.iat);

//     // Now send a request
//     let server = build_server_test();
//     thread::spawn(|| {
//         let client: BBoxClient = BBoxClient::tracked(server).unwrap();

//         // check to make sure we can connect
//         let value = "Bearer ".to_owned() + INSTR_TOKEN.clone();
//         let header = Header::new("Authorization", value);
//         let mut request = client.get("/login");
//         request.add_header(header);
//         let response = request.dispatch();
//         assert!(response.status() == Status::Ok);

//         // We got the correct user!
//         let response_body: LoginResponse = response.into_json::<LoginResponse>().unwrap();
//         assert_eq!(response_body.email, "sarah_ridley@brown.edu");
//         assert_eq!(response_body.name, "Prof. S");
//         assert_eq!(response_body.privilege, 1);
//         assert_eq!(response_body.success, true);
//     }).join().expect("Thread panicked")
// }

// #[tokio::test]
// async fn test_instr() {
//     // TEST ACTIONS BY INSTRUCTORS: 
//     // - Viewing student buffers
//     // - Registering students

//     let mock_jwk_server = setup_mock_server().await;
//     let jwk = Jwk::new(KID, JWK_N);

//     mock_jwk_issuer(vec![jwk].as_slice())
//         .mount(&mock_jwk_server)
//         .await;

//     // Send a request
//     let server = build_server_test();
//     thread::spawn(|| {
//         let client: BBoxClient = BBoxClient::tracked(server).unwrap();

//         // Check to make sure we can see student buffers
//         let value = "Bearer ".to_owned() + INSTR_TOKEN.clone();
//         let header = Header::new("Authorization", value);
//         let mut instr_request = client.get("/instructor");
//         instr_request.add_header(header.clone());
//         let instr_response = instr_request.dispatch();
//         assert!(instr_response.status() == Status::Ok);

//         let mut instr_response_body: InstructorResponse = instr_response.into_json::<InstructorResponse>().unwrap();
//         let current_num_students = instr_response_body.students.len();
//         assert!(0 < current_num_students);
//         assert_eq!(instr_response_body.class_id, 0);
//         assert!(0 < instr_response_body.student_groups.len());

//         // Check to make sure that we can register students
//         let mut reg_request = client.post("/register_student?stud_group=0&stud_name=paul&stud_class=0&stud_email=paul@gmail.com");
//         reg_request.add_header(header.clone());
//         let reg_response = reg_request.dispatch();
//         assert!(reg_response.status() == Status::Ok);
        
//         let reg_response_body: SuccessResponse = reg_response.into_json::<SuccessResponse>().unwrap();
//         assert_eq!(reg_response_body.success, true);

//         // Check to make sure that the student was registered (we have 1 more student!)
//         let mut new_instr_request = client.get("/instructor");
//         new_instr_request.add_header(header.clone());
//         let new_instr_response = new_instr_request.dispatch();
//         assert!(new_instr_response.status() == Status::Ok);

//         let new_instr_response_body = new_instr_response.into_json::<InstructorResponse>().unwrap();
//         assert_eq!(new_instr_response_body.students.len(), current_num_students + 1);

//     }).join().expect("Thread panicked")
// }

#[tokio::test]
async fn test_stud() {
    // TEST ACTIONS BY STUDENTS: 
    // - Viewing their own buffers
    // - Writing to their own buffer

    let mock_jwk_server = setup_mock_server().await;
    let jwk = Jwk::new(KID, JWK_N);

    mock_jwk_issuer(vec![jwk].as_slice())
        .mount(&mock_jwk_server)
        .await;

    // Send a request
    let server = build_server_test();
    thread::spawn(|| {
        let client: BBoxClient = BBoxClient::tracked(server).unwrap();

        // Check to make sure we can see student buffers
        let value = "Bearer ".to_owned() + STUD_TOKEN;
        let header = Header::new("Authorization", value);
        let mut stud_request = client.get("/student");
        stud_request.add_header(header.clone());
        let stud_response = stud_request.dispatch();
        assert!(stud_response.status() == Status::Ok);

        let stud_response_body: StudentResponse = stud_response.into_json::<StudentResponse>().unwrap();
        assert_eq!(stud_response_body.class_id, 0);
        assert_eq!(stud_response_body.group_id, 0);
        assert!(stud_response_body.contents.is_some());

        // Check to make sure we can write to our buffer
        let mut upd_request = client.post("/update?text=//%20This%20is%20a%20comment!");
        upd_request.add_header(header.clone());
        let upd_response = upd_request.dispatch();
        assert!(upd_response.status() == Status::Ok);
        
        let upd_response_body: SuccessResponse = upd_response.into_json::<SuccessResponse>().unwrap();
        assert_eq!(upd_response_body.success, true);

        // Check to make sure that buffer contents are updated
        let mut new_stud_request = client.get("/student");
        new_stud_request.add_header(header.clone());
        let new_stud_response = new_stud_request.dispatch();
        assert!(new_stud_response.status() == Status::Ok);

        let new_stud_response_body: StudentResponse = new_stud_response.into_json::<StudentResponse>().unwrap();
        assert_eq!(new_stud_response_body.contents.unwrap(), "// This is a comment!");

    }).join().expect("Thread panicked")
}

#[tokio::test]
async fn test_admin() {
    // TEST ACTIONS BY ADMIN: 
    // - Registering instructors
}
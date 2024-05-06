use alohomora::testing::BBoxClient;
use rocket_firebase_auth::FirebaseAuth;
use rocket::http::{Header, Status};
use std::thread;
use voltron::{build_server_test, initialize};
mod common;
use common::{JWK_N, KID, INSTR_TOKEN, STUD_TOKEN, ADMIN_TOKEN, TEST_JWKS_URL, setup_mock_server, mock_jwk_issuer, Instructor, AdminResponse, InstructorResponse, StudentResponse, SuccessResponse, Student, StudentGroup, LoginResponse};
use rocket_firebase_auth::jwk::Jwk;
use rocket::serde::Deserialize;

#[tokio::test]
async fn test_auth_state_pol() {
    // Auth state policy should stop us from returning an email address
    // directly from token (emails from tokens can only be used in db queries)
    initialize();

    let mock_jwk_server = setup_mock_server().await;
    let jwk = Jwk::new(KID, JWK_N);

    mock_jwk_issuer(vec![jwk].as_slice())
        .mount(&mock_jwk_server)
        .await;

    // Send a request
    let server = build_server_test();
    thread::spawn(|| {
        let client: BBoxClient = BBoxClient::tracked(server).unwrap();

        let value = "Bearer ".to_owned() + INSTR_TOKEN.clone();
        let header = Header::new("Authorization", value);
        let mut request = client.get("/login_auth_buggy");
        request.add_header(header);
        let response = request.dispatch();
        assert!(response.status() != Status::Ok);

    }).join().expect("Thread panicked")
}

#[tokio::test]
async fn test_email_pol() {
    // Email policy should stop us from executing a database query
    // using an email other than the one attached to our token
    initialize();

    let mock_jwk_server = setup_mock_server().await;
    let jwk = Jwk::new(KID, JWK_N);

    mock_jwk_issuer(vec![jwk].as_slice())
        .mount(&mock_jwk_server)
        .await;

    // Send a request
    let server = build_server_test();
    thread::spawn(|| {
        let client: BBoxClient = BBoxClient::tracked(server).unwrap();

        let value = "Bearer ".to_owned() + INSTR_TOKEN.clone();
        let header = Header::new("Authorization", value);
        let mut request = client.get("/login_email_buggy");
        request.add_header(header);
        let response = request.dispatch();
        assert!(response.status() != Status::Ok);

    }).join().expect("Thread panicked")
}

#[tokio::test]
async fn test_instructor_pol() {
    // Instructor policy should stop anyone but admins from
    // registering new instructors
    initialize();

    let mock_jwk_server = setup_mock_server().await;
    let jwk = Jwk::new(KID, JWK_N);

    mock_jwk_issuer(vec![jwk].as_slice())
        .mount(&mock_jwk_server)
        .await;

    // Send a request
    let server = build_server_test();
    thread::spawn(|| {
        let client: BBoxClient = BBoxClient::tracked(server).unwrap();

        let value = "Bearer ".to_owned() + INSTR_TOKEN.clone();
        let header = Header::new("Authorization", value);
        let mut reg_request = client.post("/register_instructor?instr_name=paul&instr_class=3&instr_email=paul@gmail.com");
        reg_request.add_header(header.clone());
        let reg_response = reg_request.dispatch();
        assert!(reg_response.status() == Status::InternalServerError);

    }).join().expect("Thread panicked")
}

#[tokio::test]
async fn test_read_pol() {
    // Read policy should prevent unauthorized people from reading buffers
    // that they should not have access to 
    initialize();

    let mock_jwk_server = setup_mock_server().await;
    let jwk = Jwk::new(KID, JWK_N);

    mock_jwk_issuer(vec![jwk].as_slice())
        .mount(&mock_jwk_server)
        .await;

    // Send a request
    let server = build_server_test();
    thread::spawn(|| {
        let client: BBoxClient = BBoxClient::tracked(server).unwrap();

        let value = "Bearer ".to_owned() + INSTR_TOKEN.clone();
        let header = Header::new("Authorization", value);
        let mut reg_request = client.get("/instructor_buggy");
        reg_request.add_header(header.clone());
        let reg_response = reg_request.dispatch();
        assert!(reg_response.status() == Status::InternalServerError);

    }).join().expect("Thread panicked")
}

#[tokio::test]
async fn test_student_pol() {
    // Student policy should stop us from registering students
    // to a class that is not our class (as the instructor)
    initialize();

    let mock_jwk_server = setup_mock_server().await;
    let jwk = Jwk::new(KID, JWK_N);

    mock_jwk_issuer(vec![jwk].as_slice())
        .mount(&mock_jwk_server)
        .await;

    // Send a request
    let server = build_server_test();
    thread::spawn(|| {
        let client: BBoxClient = BBoxClient::tracked(server).unwrap();

        let value = "Bearer ".to_owned() + INSTR_TOKEN.clone();
        let header = Header::new("Authorization", value);
        let mut reg_request = client.post("/register_student_buggy?stud_group=0&stud_name=paul&stud_class=0&stud_email=paul@gmail.com");
        reg_request.add_header(header.clone());
        let reg_response = reg_request.dispatch();
        assert!(reg_response.status() == Status::InternalServerError);

    }).join().expect("Thread panicked")
}

#[tokio::test]
async fn test_write_pol() {
    // Write policy should prevent unauthorized people from writing to
    // buffers that they should not have access to 
    initialize();

    let mock_jwk_server = setup_mock_server().await;
    let jwk = Jwk::new(KID, JWK_N);

    mock_jwk_issuer(vec![jwk].as_slice())
        .mount(&mock_jwk_server)
        .await;

    // Send a request
    let server = build_server_test();
    thread::spawn(|| {
        let client: BBoxClient = BBoxClient::tracked(server).unwrap();

        let value = "Bearer ".to_owned() + STUD_TOKEN.clone();
        let header = Header::new("Authorization", value);
        let mut reg_request = client.post("/update_buggy?text=//%20This%20is%20a%20comment!");
        reg_request.add_header(header.clone());
        let reg_response = reg_request.dispatch();
        assert!(reg_response.status() == Status::InternalServerError);

    }).join().expect("Thread panicked")
}
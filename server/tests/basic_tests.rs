use alohomora::testing::BBoxClient;
use rocket::http::{ContentType, Header, Status};
use rocket::response::{Response, Body};
use std::thread;
use std::io::Read;

use voltron::build_server_test;
mod common;
use common::{JWK_N, KID, INSTR_TOKEN, TEST_JWKS_URL, setup_mock_server, mock_jwk_issuer};
use once_cell::sync::Lazy;
use rocket_firebase_auth::{
    errors::{Error, InvalidJwt},
    jwk::Jwk,
    FirebaseAuth,
};

use rocket::serde::{Deserialize, Serialize};
use serde_json::json;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

#[derive(Deserialize)]
struct LoginResponse {
    email: String,
    name: String,
    privilege: i32,
    success: bool
}

// RUN WITH RUST_TEST_THREADS=1 cargo test

#[tokio::test]
async fn test_login() {
    let mock_jwk_server = setup_mock_server().await;
    //let scenario = SCENARIO_PROF.clone();
    //let jwk = Jwk::new(scenario.kid.as_str(), scenario.jwk_n.as_str());
    let jwk = Jwk::new(KID, JWK_N);

    mock_jwk_issuer(vec![jwk].as_slice())
        .mount(&mock_jwk_server)
        .await;

    let firebase_auth = FirebaseAuth::builder()
        .json_file("./tests/dummy-firebase-creds.json")
        .jwks_url(TEST_JWKS_URL)
        .build()
        .unwrap();
    //let decoded_token = firebase_auth.verify(scenario.token.as_str()).await;
    let decoded_token = firebase_auth.verify(INSTR_TOKEN).await;

    assert!(decoded_token.is_ok());

    let decoded_token = decoded_token.unwrap();

    assert_eq!(decoded_token.sub, "some-uid");
    assert_eq!(decoded_token.email, Some("sarah_ridley@brown.edu".to_string()));
    assert!(decoded_token.exp > decoded_token.iat);

    // Now send a request
    let server = build_server_test();
    thread::spawn(|| {
        let client: BBoxClient = BBoxClient::tracked(server).unwrap();

        // check to make sure we can connect
        //let value = "Bearer ".to_owned() + SCENARIO_PROF.clone().token.as_str();
        let value = "Bearer ".to_owned() + INSTR_TOKEN.clone();
        let header = Header::new("Authorization", value);
        let mut request = client.get("/login");
        request.add_header(header);
        let response = request.dispatch();
        assert!(response.status() == Status::Ok);

        // We got the correct user!
        let response_body: LoginResponse = response.into_json::<LoginResponse>().unwrap();
        assert_eq!(response_body.email, "sarah_ridley@brown.edu");
        assert_eq!(response_body.name, "Prof. S");
        assert_eq!(response_body.privilege, 1);
        assert_eq!(response_body.success, true);
    }).join().expect("Thread panicked")
}

#[tokio::test]
async fn test_instr() {
    // TEST ACTIONS BY INSTRUCTORS: 
    // - Viewing student buffers
    // - Registering students
}

#[tokio::test]
async fn test_stud() {
    // TEST ACTIONS BY STUDENTS: 
    // - Viewing their own buffers
    // - Writing to their own buffer
}

#[tokio::test]
async fn test_admin() {
    // TEST ACTIONS BY ADMIN: 
    // - Registering instructors
}
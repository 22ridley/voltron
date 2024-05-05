use alohomora::testing::BBoxClient;
use rocket::http::{ContentType, Header, Status};
use rocket::response::{Response, Body};
use std::thread;
use std::io::Read;

use voltron::build_server_jwk;
mod common;
use common::{SCENARIO_PROF, TEST_JWKS_URL, setup_mock_server, mock_jwk_issuer};
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
    let scenario = SCENARIO_PROF.clone();
    let jwk = Jwk::new(scenario.kid.as_str(), scenario.jwk_n.as_str());

    mock_jwk_issuer(vec![jwk].as_slice())
        .mount(&mock_jwk_server)
        .await;

    let firebase_auth = FirebaseAuth::builder()
        .json_file("./src/dummy-firebase-creds.json")
        .jwks_url(TEST_JWKS_URL)
        .build()
        .unwrap();
    let decoded_token = firebase_auth.verify(scenario.token.as_str()).await;

    assert!(decoded_token.is_ok());

    let decoded_token = decoded_token.unwrap();

    assert_eq!(decoded_token.sub, "some-uid");
    assert_eq!(decoded_token.email, Some("sarah_ridley@brown.edu".to_string()));
    assert!(decoded_token.exp > decoded_token.iat);

    // Now send a request
    let server = build_server_jwk();
    thread::spawn(|| {
        let client: BBoxClient = BBoxClient::tracked(server).unwrap();

        // check to make sure we can connect
        let value = "Bearer eyJhbGciOiJSUzI1NiIsImtpZCI6Ijc2MDI3MTI2ODJkZjk5Y2ZiODkxYWEwMzdkNzNiY2M2YTM5NzAwODQiLCJ0eXAiOiJKV1QifQ.eyJuYW1lIjoiU2FyYWggUmlkbGV5IiwicGljdHVyZSI6Imh0dHBzOi8vbGgzLmdvb2dsZXVzZXJjb250ZW50LmNvbS9hL0FDZzhvY0t3enFCMzZGMS1WWWZyT2REUnp0X3JlZjJ0ZENSRlBHa2hKSlNfa1FMQj1zOTYtYyIsImlzcyI6Imh0dHBzOi8vc2VjdXJldG9rZW4uZ29vZ2xlLmNvbS92b2x0cm9uLTFlYTVjIiwiYXVkIjoidm9sdHJvbi0xZWE1YyIsImF1dGhfdGltZSI6MTcxNDYxMjYzMywidXNlcl9pZCI6IlpZR29ndnd1bnRNWUp4bGdvelNXdWs4RkNmQzIiLCJzdWIiOiJaWUdvZ3Z3dW50TVlKeGxnb3pTV3VrOEZDZkMyIiwiaWF0IjoxNzE0NzUyMjE4LCJleHAiOjE3MTQ3NTU4MTgsImVtYWlsIjoic2FyYWhfcmlkbGV5QGJyb3duLmVkdSIsImVtYWlsX3ZlcmlmaWVkIjp0cnVlLCJmaXJlYmFzZSI6eyJpZGVudGl0aWVzIjp7Imdvb2dsZS5jb20iOlsiMTA1NzI4MzU3MzEzMjA1OTk4ODU2Il0sImVtYWlsIjpbInNhcmFoX3JpZGxleUBicm93bi5lZHUiXX0sInNpZ25faW5fcHJvdmlkZXIiOiJnb29nbGUuY29tIn19.XwxlXdkvcdVstZ3O_DXoDv4_03m1CBOCV2Uutq-SpAZOY2iEsf_yCuVPagbgt8Pu-oIMfJTf5aLnjktyHefhVr6YCeD0xTo8xF-yPSNrwKDI1uf_ftQCdhuS5fLLynH9I74aE5tM8nfXY20_iUbO3a-XHFQRwovlMYwGRhT2X461m-nnlGHeVhxcnG5l1hg1vnm8E8ZEkttrOGq49TDzzpbourOxgnZqSkzIjrT-a-rXy02LDep_E7o92J9anMbIk_rorJ29q0tNAAJg4QhljPA5O_EcGITqVS1rxVYM-ufhIAJDy0AoCkUmDe_qSH_USJ5VLaaQf3pTfz6vzY-Fbw";
        let value = "Bearer ".to_owned() + SCENARIO_PROF.clone().token.as_str();
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
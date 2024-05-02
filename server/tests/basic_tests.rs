use alohomora::testing::BBoxClient;
use rocket::http::ContentType;

use voltron::build_server;

#[test]
fn test_connect() {
    let server = build_server();
    let client: BBoxClient = BBoxClient::tracked(server).unwrap();

    // check to make sure we're redirected from root to login
    // assert!(response_redirects(&client, "/".to_string(), "/login"));
}
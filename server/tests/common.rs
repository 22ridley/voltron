use alohomora::testing::BBoxClient;
use rocket::http::Status;

#[allow(dead_code)]
// uses "client" to check if a get request to "uri" results in a redirect to "desired_dest"
pub fn response_redirects(client: &BBoxClient, uri: String, desired_dest: &str)
    -> bool {
    // make sure we get redirected (w/ a 3XX HTTP response code)
    let response = client.get(uri).dispatch();
    assert!(is_redirect(response.status()));
    
    // make sure it's to the desired page
    let dest = response.headers().get_one("Location").unwrap().to_string();
    dest == desired_dest
}

#[allow(dead_code)]
// tests if a given status is a valid HTTP redirect status
pub(crate) fn is_redirect(status: Status) -> bool{
    (status.code / 100) == 3
}

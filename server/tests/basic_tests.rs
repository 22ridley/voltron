use alohomora::testing::BBoxClient;
use rocket::http::{ContentType, Header, Status};
use rocket::response::{Response, Body};
use std::thread;

use voltron::build_server_jwk;
mod common;
use common::response_redirects;
use once_cell::sync::Lazy;
use rocket_firebase_auth::{
    errors::{Error, InvalidJwt},
    jwk::Jwk,
    FirebaseAuth,
};

use serde::{Deserialize, Serialize};
use serde_json::json;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

// RUN WITH RUST_TEST_THREADS=1 cargo test

#[tokio::test]
async fn test_connect() {
    let mock_jwk_server = setup_mock_server().await;
    let scenario = SCENARIO_HAPPY_PATH.clone();
    let jwk = Jwk::new(scenario.kid.as_str(), scenario.jwk_n.as_str());

    mock_jwk_issuer(vec![jwk].as_slice())
        .expect(1)
        .mount(&mock_jwk_server)
        .await;

    let firebase_auth = FirebaseAuth::builder()
        //.json_file("./src/firebase-credentials.json")
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
        let value = "Bearer ".to_owned() + SCENARIO_HAPPY_PATH.clone().token.as_str();
        let header = Header::new("Authorization", value);
        let mut request = client.get("/login");
        request.add_header(header);
        let response = request.dispatch();
        assert!(response.status() == Status::Ok);
    }).join().expect("Thread panicked")
}

// #[test]
// async fn should_succeed_with_env() {
//     let mock_server = setup_mock_server().await;
//     let scenario = SCENARIO_HAPPY_PATH.clone();
//     let jwk = Jwk::new(scenario.kid.as_str(), scenario.jwk_n.as_str());

//     mock_jwk_issuer(vec![jwk].as_slice())
//         .expect(1)
//         .mount(&mock_server)
//         .await;

//     let firebase_auth = FirebaseAuth::builder()
//         .env_file("tests/env_files/.env", "FIREBASE_CREDS")
//         .jwks_url(TEST_JWKS_URL)
//         .build()
//         .unwrap();
//     let decoded_token = firebase_auth.verify(scenario.token.as_str()).await;

//     assert!(decoded_token.is_ok());

//     let decoded_token = decoded_token.unwrap();

//     assert_eq!(decoded_token.sub, "some-uid");
//     assert!(decoded_token.exp > decoded_token.iat);
// }

// HELPERS

static SCENARIO: Lazy<Scenario> = Lazy::new(|| {
    Scenario {
        //token: "eyJhbGciOiJSUzI1NiIsImtpZCI6Ijc2MDI3MTI2ODJkZjk5Y2ZiODkxYWEwMzdkNzNiY2M2YTM5NzAwODQiLCJ0eXAiOiJKV1QifQ.eyJuYW1lIjoiU2FyYWggUmlkbGV5IiwicGljdHVyZSI6Imh0dHBzOi8vbGgzLmdvb2dsZXVzZXJjb250ZW50LmNvbS9hL0FDZzhvY0t3enFCMzZGMS1WWWZyT2REUnp0X3JlZjJ0ZENSRlBHa2hKSlNfa1FMQj1zOTYtYyIsImlzcyI6Imh0dHBzOi8vc2VjdXJldG9rZW4uZ29vZ2xlLmNvbS92b2x0cm9uLTFlYTVjIiwiYXVkIjoidm9sdHJvbi0xZWE1YyIsImF1dGhfdGltZSI6MTcxNDYxMjYzMywidXNlcl9pZCI6IlpZR29ndnd1bnRNWUp4bGdvelNXdWs4RkNmQzIiLCJzdWIiOiJaWUdvZ3Z3dW50TVlKeGxnb3pTV3VrOEZDZkMyIiwiaWF0IjoxNzE0NzUyMjE4LCJleHAiOjE3MTQ3NTU4MTgsImVtYWlsIjoic2FyYWhfcmlkbGV5QGJyb3duLmVkdSIsImVtYWlsX3ZlcmlmaWVkIjp0cnVlLCJmaXJlYmFzZSI6eyJpZGVudGl0aWVzIjp7Imdvb2dsZS5jb20iOlsiMTA1NzI4MzU3MzEzMjA1OTk4ODU2Il0sImVtYWlsIjpbInNhcmFoX3JpZGxleUBicm93bi5lZHUiXX0sInNpZ25faW5fcHJvdmlkZXIiOiJnb29nbGUuY29tIn19.XwxlXdkvcdVstZ3O_DXoDv4_03m1CBOCV2Uutq-SpAZOY2iEsf_yCuVPagbgt8Pu-oIMfJTf5aLnjktyHefhVr6YCeD0xTo8xF-yPSNrwKDI1uf_ftQCdhuS5fLLynH9I74aE5tM8nfXY20_iUbO3a-XHFQRwovlMYwGRhT2X461m-nnlGHeVhxcnG5l1hg1vnm8E8ZEkttrOGq49TDzzpbourOxgnZqSkzIjrT-a-rXy02LDep_E7o92J9anMbIk_rorJ29q0tNAAJg4QhljPA5O_EcGITqVS1rxVYM-ufhIAJDy0AoCkUmDe_qSH_USJ5VLaaQf3pTfz6vzY-Fbw".to_string(),
        token: "eyJhbGciOiJSUzI1NiIsImtpZCI6Ijc2MDI3MTI2ODJkZjk5Y2ZiODkxYWEwMzdkNzNiY2M2YTM5NzAwODQiLCJ0eXAiOiJKV1QifQ.eyJuYW1lIjoiU2FyYWggUmlkbGV5IiwicGljdHVyZSI6Imh0dHBzOi8vbGgzLmdvb2dsZXVzZXJjb250ZW50LmNvbS9hL0FDZzhvY0t3enFCMzZGMS1WWWZyT2REUnp0X3JlZjJ0ZENSRlBHa2hKSlNfa1FMQj1zOTYtYyIsImlzcyI6Imh0dHBzOi8vc2VjdXJldG9rZW4uZ29vZ2xlLmNvbS92b2x0cm9uLTFlYTVjIiwiYXVkIjoidm9sdHJvbi0xZWE1YyIsImF1dGhfdGltZSI6MTcxNDYxMjYzMywidXNlcl9pZCI6IlpZR29ndnd1bnRNWUp4bGdvelNXdWs4RkNmQzIiLCJzdWIiOiJaWUdvZ3Z3dW50TVlKeGxnb3pTV3VrOEZDZkMyIiwiaWF0IjoxNzE0NzUyMjE4LCJleHAiOjUwMTQ3NTU4MTgsImVtYWlsIjoic2FyYWhfcmlkbGV5QGJyb3duLmVkdSIsImVtYWlsX3ZlcmlmaWVkIjp0cnVlLCJmaXJlYmFzZSI6eyJpZGVudGl0aWVzIjp7Imdvb2dsZS5jb20iOlsiMTA1NzI4MzU3MzEzMjA1OTk4ODU2Il0sImVtYWlsIjpbInNhcmFoX3JpZGxleUBicm93bi5lZHUiXX0sInNpZ25faW5fcHJvdmlkZXIiOiJnb29nbGUuY29tIn19.rtSyIyf9luiwRsPgU-h1Deku0qWqi93t9F4HHomEgfQdGmenpUGIG3yTsUqiH1xV-kthckS2ZfqgNoaEwfaM8NmONVFpXbwBPiqVcoRcwmNAe57DJJF9quO3qvlJHbMfqDMBaLSYnWdOnimmfRtsiFaVwN2FSYrwMRl7e0k0YIP5DiMeWMc0zGMlNLENXQXzTDLePnCYsDDuaaPXoyi_LRiE_JZp9-fo5Su6wcWhwS1JagR5eQiLqTiQkRLVO_Xo_QGKs-GbLXTTzxJb6z59kcNV5Q1kpiMTWUe_PWkill1xbYDQwewMcO2NabmqQgoREEgAlyCeHa4PC_v0u66z_w".to_string(),
        jwk_n: "meR-imFrj9fksArDfbUF7A-CV4ouxRCa_zRqEJx3jwaG6zsx2kncsaL-6N16raLl5kQ3rVaQDXaYdq-fq0xWNdt9SnoUBfR6LGAwOM0_z4d9mfpKUvV5ZIPudeZaMELWBp8JQJpx4q4r04FJqx0exmlVTqOyuXqw9bWxYiI04LRzUY5cVUu_OlhHbzpFaHSUHCl-f-RQ5SNNQAG52y3_oiqQqv21zRhUCP-q-O006PahJlAB6D2qlti3HhqR9aqqAaAUX7EuKmlmwMbB47NHLq-MeAItXPoiWxTR0H24rPwczOXeQywxVeJ92T4ntN2iaKJ1m4iV9Xk1tTaOF8qM3w".to_string(),
        kid: "7602712682df99cfb891aa037d73bcc6a3970084".to_string(),
        // kid: "0922171885ba7cffaf40b2d4afd363d259e4e633",
        public_key: Some("-----BEGIN CERTIFICATE-----\nMIIDHTCCAgWgAwIBAgIJALeM3DBeoI3wMA0GCSqGSIb3DQEBBQUAMDExLzAtBgNV\nBAMMJnNlY3VyZXRva2VuLnN5c3RlbS5nc2VydmljZWFjY291bnQuY29tMB4XDTI0\nMDQzMDA3MzIyMVoXDTI0MDUxNjE5NDcyMVowMTEvMC0GA1UEAwwmc2VjdXJldG9r\nZW4uc3lzdGVtLmdzZXJ2aWNlYWNjb3VudC5jb20wggEiMA0GCSqGSIb3DQEBAQUA\nA4IBDwAwggEKAoIBAQCZ5H6KYWuP1+SwCsN9tQXsD4JXii7FEJr/NGoQnHePBobr\nOzHaSdyxov7o3XqtouXmRDetVpANdph2r5+rTFY1231KehQF9HosYDA4zT/Ph32Z\n+kpS9Xlkg+515lowQtYGnwlAmnHirivTgUmrHR7GaVVOo7K5erD1tbFiIjTgtHNR\njlxVS786WEdvOkVodJQcKX5/5FDlI01AAbnbLf+iKpCq/bXNGFQI/6r47TTo9qEm\nUAHoPaqW2LceGpH1qqoBoBRfsS4qaWbAxsHjs0cur4x4Ai1c+iJbFNHQfbis/BzM\n5d5DLDFV4n3ZPie03aJoonWbiJX1eTW1No4XyozfAgMBAAGjODA2MAwGA1UdEwEB\n/wQCMAAwDgYDVR0PAQH/BAQDAgeAMBYGA1UdJQEB/wQMMAoGCCsGAQUFBwMCMA0G\nCSqGSIb3DQEBBQUAA4IBAQBGERUt+83Ar/OjpwpG9n1hsgM5X5TBrZXMPLpzlr0Y\nDOSB3svrvwBOcJftddUIStJKaEaFwuK+N6TuxtYbcE8tBF7QG1H1M7OdIb8j1o4j\naGggP9ziXiFgRHBADd8o4gHgeBygfZQUU73XHDu1jSzNsUELF0mUt5ffKxSoRtq2\ne1ng74n9sBmExN7HNW8DnyXyF21AnFeCqY3ttTY4KttsGKIXJB1PKXZ31wbTTeVH\njmn+QRC6co2ENNCgCtWr1GiBrgkve8HbtR1qbSDnpBiGAdH+yxBWCRNTEEPW4E7b\nZTGhgbFh1YNFf/+ihvomrfCeCdfwbQEkvs6hhQAI4nTC\n-----END CERTIFICATE-----\n".to_string()),
        private_key: Some("-----BEGIN PRIVATE KEY-----\nMIIEvAIBADANBgkqhkiG9w0BAQEFAASCBKYwggSiAgEAAoIBAQC4Z4wOp4PhsLKb\n0euTbjIxvG5J7Nvf9rVI9IlmO35hHC5McXKa8LzfawD/fFRg2eiUT0/DJ4SNZtUb\n3miLKUn0FwLAAq5QtnRCHez+I6XnnpvJZMBbkUQS4tIIj0U1DPPPp8zTY3GVM+Re\nHx3Si5mu8uyesQDoczOdSQIep17slIxgztuEUTdfulEBTZJ9oyyBbKztlsnZ9vVp\nqnfRU2sM6Tu2Voi0yIiAyP1Fh+WUfTpqoGlSgPveqxEfLEndMlO1r2bbBnXDYjkY\n07YMBMkWi0L946DUipMx/A0HXJ1zPRrjZZZmxPUGFx9CqboSrGER8kvgQzJClBaZ\nF7EOmkR7AgMBAAECggEAO6d7vo3gcxVnYlMRg3slj8NmZlHSe2xjDvvUqxbZ9m5Y\nZ/BXY9o3L4t9IilFXqN/wuK5aeeR0PCQRli3tzUfMHB1Okpju8aaa30k2r4LgJdl\nzEvkknq26+kmixBWEihjgyzEbSoBZ4QG/4/PZPo2OEtr0pZuNcR20Q36OpJoed2z\nFmtAdbgBujdgY6r2JCqHiS/g69K0vl3ucoSkL1XKlmBZpfx7q3P4GK6wQUDbp9VR\nON4mKLeGP1hAU++HFmpjuc4z9dy0Xn26fol9tZlb1D5fKFpWIWCQfs2EfcSiLseZ\nq+tAWVcziAJfWqsFZH6nSqRhxo7FnuWapLSrPE2SYQKBgQDwP5DHeHn36a1VxiKq\ncUmGc8x0SuWhbvtl3/FM9YbGDMXhciJkJ+VcCzGZW3gMgtuWl7PshfbdLPg3gazY\ntvL5WBDEDbrNRtL69V11+P6JWXAD9Q/C+jIy9a1fVckLlRpUReC4wLbgJGSNNv9U\n+8Tt0VdiHhw8tqtQ8eYGJwX5cQKBgQDEfqyKfURNkvHXfScX07wlsDizqIEqVkEE\njOj2paG1BZvZBlF2/IlLJVgbQqRbtgHjaHqiBwvXS/xXhJniwDJb0wyITw51snkJ\ng0zzfoOtimOw2AC4EvGoAR2y+qDK2zRKg+i41lLl4jGbLHf1EHrNhVC1MnRI4SGX\nDWRoNAQGqwKBgCSdS29bCRujXWtpGvdm9JGtLEe88mo8A8xu6NOVx8h/QdkuD9e7\nzNTOT+zTZ+RgPR8iV6WAdTMYG2w6MyGEIFun9EYs4X9Pf/AHXQsRu5krLEsuGgzZ\nHLVvg9C3vkAppS2ayLjfhf97DGgJLbjB9i1Ybjzh4ePO4yUe+LOPVvXBAoGABAbQ\nhFoCz1AmkpLm/S8CmQceclgOpY508KHumDdWs/30bqMZgkW0joyqXIymD5cQPNfE\nQ5O/Y18UQbre6G2l2ondpOFx22mX32EIb6j1C9ZdKp+SjDMfLqEvOZ63MtEBJlHD\nsB38Pue+un5TyvrWeWW0/LGW/nNWm+DTYOtH9dcCgYB+ciejpE4irtBotoiW2QZL\nFERzJFY8gof/vLRbuyZhgLG/gm5fnWUhx0iRcyM0knSWCWDm2xKeQ2jj3RGdVGRk\nSLUCeMo8MF8piBDpgAT9Rj2zaqwGI+B7B1M03e5enZeLpbeb4Ke9Wx5eCagM024Z\ndoDFYwUmOYE0PhGGrCaDpw==\n-----END PRIVATE KEY-----\n".to_string()),
    }
});

static SCENARIO_HAPPY_PATH: Lazy<Scenario> = Lazy::new(|| {
    Scenario {
        //token: "eyJhbGciOiJSUzI1NiIsImtpZCI6InRlc3Rfa2lkIn0.eyJzdWIiOiJzb21lLXVpZCIsImF1ZCI6ImR1bW15LXByb2plY3QtaWQiLCJpYXQiOjE2NjcwOTc2MjMsImV4cCI6NTAwNzA5OTYyMywiaXNzIjoiaHR0cHM6Ly9zZWN1cmV0b2tlbi5nb29nbGUuY29tL2R1bW15LXByb2plY3QtaWQifQ.A0ljz9xFb9p2CC6MzOlm-In-1wLIbbZVx3YgSTeajxEIYdrqNBVp-wh00mfwPM9ZGtuPmOUP8wbEzY_XjClNFCWdExmRY-UTLga0IeAekDgo8uWgr4g3cVQ5-RSK-mSDl1svzaZBq2YYyWpYZkfcsp2eoJhS04A4t9d_ADnvzNTC9YlPAyDp6DeDvyUE9K6B6_xIj2murScOz9CKijJq1rzItl1Ip8JVlNrEVFeiF5a7MZTMZYkV6ZN31hOCqc1N4eIhxHJ3ZvvM6YbZva-pdFsu_XGroYIb4Ar_hDpa3i4KrM517mp9e_nocGFKkjfQpNUnp0lsJ_PzRZZvHZ5rrQ".to_string(),
        token: "eyJhbGciOiJSUzI1NiIsImtpZCI6InRlc3Rfa2lkIn0.eyJzdWIiOiJzb21lLXVpZCIsImF1ZCI6ImR1bW15LXByb2plY3QtaWQiLCJpYXQiOjE2NjcwOTc2MjMsImV4cCI6NTAwNzA5OTYyMywiZW1haWwiOiJzYXJhaF9yaWRsZXlAYnJvd24uZWR1IiwiaXNzIjoiaHR0cHM6Ly9zZWN1cmV0b2tlbi5nb29nbGUuY29tL2R1bW15LXByb2plY3QtaWQifQ.RCRV9iT6245UMQEJ8App7rkMHqicLDkPC4ENgy3wxPECLRltpNfA6PADUZ_krxqcKto93thJjjAtujIAj7BXIhADOt2WPGCWcG1pC1zIZxNZ3mPBNQ62WuZNn7aoUAOxU6YuFHLXQP99goZmyw3fAX5N2PP5TWKfl8XoVAL2bl_s8WG5ogqJCCVfx3zM4fzJSbGuIphL51r8DjF_mPmfBs-EoBC1N8z8xZXKyVn0gR0aZM_lWb2ClJ-RNvW5yNdvlr_SwjUEXW6iZ1UL5tq-C-_Vae0hNzTaey-Uu98dEQk6STOR23ZZWtEwCzIr6hCUn1xkKpvdgmwPSP7JNqsllg".to_string(),
        jwk_n: "XiqTPp559nQUm1dxEecu1HT_yD0Vv4OY9uVG-YNCy8NU2ceaqoUfF05luJ9xtmf-D3PQjpwSouA2ejxr_GSD2VQ99our7CDe5t5jdNEXyXSrXJDua5-lEETT-fOmnAuhti9m86w7Jd2HafJCq9sGEuQc4-CM1-2faerEGGl9iyNGQEZBnS_ExDJOJnbI8J6JZnWhL_rjwkVcM6_MX9bDA1Hi4fosMW1V-WhnZUJ5-WvNfX_2feXfQ0qq5kH9BWhbYCxwwT2d_0xkdL0aItEB_nqZSS9yIDiGQATjf_P708dEo9A0tVZg9HUtLY5k4-JCrOUayJOYJ5b-YGETzSbELw".to_string(),
        kid: "test_kid".to_string(),
        public_key: Some(r#"-----BEGIN RSA PUBLIC KEY-----
MIIBITANBgkqhkiG9w0BAQEFAAOCAQ4AMIIBCQKCAQBeKpM+nnn2dBSbV3ER5y7UdP/IPRW/g5j25Ub5g0LLw1TZx5qqhR8XTmW4n3G2Z/4Pc9COnBKi4DZ6PGv8ZIPZVD32i6vsIN7m3mN00RfJdKtckO5rn6UQRNP586acC6G2L2bzrDsl3Ydp8kKr2wYS5Bzj4IzX7Z9p6sQYaX2LI0ZARkGdL8TEMk4mdsjwnolmdaEv+uPCRVwzr8xf1sMDUeLh+iwxbVX5aGdlQnn5a819f/Z95d9DSqrmQf0FaFtgLHDBPZ3/TGR0vRoi0QH+eplJL3IgOIZABON/8/vTx0Sj0DS1VmD0dS0tjmTj4kKs5RrIk5gnlv5gYRPNJsQvAgMBAAE=
-----END RSA PUBLIC KEY-----"#.to_string()),
        private_key: Some(r#"-----BEGIN RSA PRIVATE KEY-----
MIIEoQIBAAKCAQBeKpM+nnn2dBSbV3ER5y7UdP/IPRW/g5j25Ub5g0LLw1TZx5qq
hR8XTmW4n3G2Z/4Pc9COnBKi4DZ6PGv8ZIPZVD32i6vsIN7m3mN00RfJdKtckO5r
n6UQRNP586acC6G2L2bzrDsl3Ydp8kKr2wYS5Bzj4IzX7Z9p6sQYaX2LI0ZARkGd
L8TEMk4mdsjwnolmdaEv+uPCRVwzr8xf1sMDUeLh+iwxbVX5aGdlQnn5a819f/Z9
5d9DSqrmQf0FaFtgLHDBPZ3/TGR0vRoi0QH+eplJL3IgOIZABON/8/vTx0Sj0DS1
VmD0dS0tjmTj4kKs5RrIk5gnlv5gYRPNJsQvAgMBAAECggEAXPsdOY+yTjCAyIKn
G05zZ0W/6zCl8N04hVIPqwB5TEor1n7JseaQtKqstoh59+rnasqo/KgPntRV9o0C
880sg8QzCucPc7FhaAXfntF382xIaLaTNaIFkvLjfMOhmCPEIejcd29xWApOU8br
Hla+wJiODlUDvZLc/fDagGBpnqCZ9YrJnnGN3Z+arJ4NBD3FeiTJdjC/phJEioWZ
gPpA7Lzkst18oIlWIPMW44B1Ng4oaBv2cyIdcQSvtsOrZoeGwdmIOz2lFI73GPfA
d5orQeTEPMAQJ31vj8yTyTtT0sq8wEX6RpW7/Q3paHPcDlv1FXpL1G1FXu62vqza
6px+AQKBgQCrazVmMYDK7mGogeKPOxaF9SIPR0n4GK3/XonUPUdxsPcrUWHKDG8K
LhTMKV+o86eHZjoFrbpCIvgT7hBzvuL7VyI84neZPLkmAOVG0l2gk3++BvLGXWb4
/Ft6FT1BGhp3OLy7YpM5aPof0PKCfeFLjzFHRwTtAAiBBWg27W6SIQKBgQCMoTUx
NpjukmIhn2rjBDJuQTZwik7qXpwZd4Z6wGefp37yv1Sgj+Bcx7bCi9TLLG1X0hLV
wf1n0ZsXmsmAP0MVX1ezO/QQpqVM6nbJiaDIs1vNK17EmzyOoteinQF+UR2xPV2d
al8jCjEAjQJn4W47X/nDTrHanBmHNUv+3aosTwKBgBaVRzGxb+BMS31hrzFjfXIk
e1o78BjJV5L/J3VYpWLrB4UjcZimzrIuo/rJsJqXjwidhSNeYd14seoeQPieu1SV
hCM1SsBbaaECGTKdYExZYkjsrWtIvtoqlPqedbVv9PCj/ulI8VBs7hbm9iwO3XGQ
6dMUHigDCxvEVJh360tBAoGAKflf8A10vhiRE6oKdDHnf4MVZafSgB+3Bd7oE7Fj
/II44Ol8r+Phuq+dfBnSbMYY6NJ57rVVFmy4luYLaKz5L+LiQUwOv/2NbxS4WdUr
WVw3dViRk6sl+wjdxdqI/JPngeRoEbkTJlk/YQO1iR3/EdfGq6XMbgyTjgi5Yxv0
U/8CgYArvXTFLe+Ia02aTYwIP3eDS9Jxd/Hxk2TpfUf0VDpjBHxqmixYW3hPgzrJ
milaoy6NhM3U0c+8WxGoEZHcQVolPkRdAkCPCzhUNQPSyzRjgSo3mPGNPBFRAre3
Cm9flz88RbOLF/TuSWk/jsh7TtSXt8jL/bR4EdXlwqTFoexpXg==
-----END RSA PRIVATE KEY-----"#.to_string()),
    }
});

#[allow(dead_code)]
#[derive(Clone, Default)]
struct Scenario {
    pub token: String,
    pub jwk_n: String,
    pub kid: String,
    pub public_key: Option<String>,
    pub private_key: Option<String>,
}

pub static TEST_JWKS_URL: &str = "http://localhost:8888/jwks_url";

pub async fn setup_mock_server() -> MockServer {
    let listener = std::net::TcpListener::bind("localhost:8888").unwrap();
    MockServer::builder().listener(listener).start().await
}

#[derive(Debug, Serialize, Deserialize)]
struct MockJwksResponse {
    pub keys: Vec<Jwk>,
}

pub fn mock_jwk_issuer(jwks: &[Jwk]) -> Mock {
    for jwk in jwks.to_vec().iter() {
        println!("JWK: {}, {}", jwk.n, jwk.e);
    }
    Mock::given(method("GET"))
        .and(path("/jwks_url"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!(MockJwksResponse {
                keys: jwks.to_vec()
            })),
        )
}
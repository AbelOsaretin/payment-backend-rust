use axum::{
    Json, Router,
    body::Bytes,
    extract::Path,
    http::StatusCode,
    response::Redirect,
    routing::{get, post},
};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

use hex::encode;
use hmac::{Hmac, Mac};
use sha2::Sha512;

//* ============================================================================
//* STRUCTS: Data structures for request/response bodies
//* ============================================================================

#[derive(Debug, Serialize, Deserialize)]
struct InitializeRequest {
    email: String,
    amount: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct InitializeResponse {
    status: bool,
    message: String,
    data: InitializeDataResponse,
}

#[derive(Debug, Serialize, Deserialize)]
struct InitializeDataResponse {
    authorization_url: String,
    access_code: String,
    reference: String,
}

#[derive(Deserialize, Debug)]
pub struct VerifyResponse {
    pub status: bool,
    pub message: String,
    pub data: VerifyResponseData,
}

#[derive(Deserialize, Debug)]
pub struct VerifyResponseData {
    pub status: String,
    pub amount: u64,
    pub reference: String,
    pub gateway_response: String,
}

//* ============================================================================
//* HANDLERS: Functions that process incoming requests and generate responses
//* ============================================================================

async fn health_check() -> (StatusCode, &'static str) {
    (StatusCode::OK, "OK 🦀")
}

async fn not_found() -> (axum::http::StatusCode, &'static str) {
    (axum::http::StatusCode::NOT_FOUND, "404 - Route not found")
}

async fn echo(body: String) -> String {
    format!("You sent: {}", body)
}

async fn initialize_payment(
    Json(payload): Json<InitializeRequest>,
) -> Result<Json<InitializeResponse>, StatusCode> {
    let api_key = std::env::var("PAYSTACK_API_KEY").unwrap();
    let url = std::env::var("PAYSTACK_INITIALIZE_URL").unwrap();

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", api_key))
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .headers(headers)
        .json(&payload)
        .send()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let paystack_res: InitializeResponse = response
        .json()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(InitializeResponse {
        status: paystack_res.status,
        message: paystack_res.message,
        data: InitializeDataResponse {
            authorization_url: paystack_res.data.authorization_url,
            access_code: paystack_res.data.access_code,
            reference: paystack_res.data.reference,
        },
    }))
}

async fn initialize_payment_redirect(
    Path(payload): Path<InitializeRequest>,
) -> Result<Redirect, StatusCode> {
    let api_key = std::env::var("PAYSTACK_API_KEY").unwrap();
    let url = std::env::var("PAYSTACK_INITIALIZE_URL").unwrap();

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", api_key))
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .headers(headers)
        .json(&payload)
        .send()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let paystack_res: InitializeResponse = response
        .json()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to(paystack_res.data.authorization_url.as_str()))
}

// async fn verify_payment_old(
//     Path(reference): Path<String>,
// ) -> Result<Json<VerifyResponse>, StatusCode> {
//     let api_key = std::env::var("PAYSTACK_API_KEY").unwrap();

//     let url_main = std::env::var("PAYSTACK_TRANS_VERIFY_URL").unwrap();

//     let url = format!("{}{}", url_main, reference);

//     // let mut headers = HeaderMap::new();
//     // headers.insert(
//     //     AUTHORIZATION,
//     //     HeaderValue::from_str(&format!("Bearer {}", api_key))
//     //         .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
//     // );
//     // headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

//     let client = reqwest::Client::new();
//     // let response = client
//     //     .get(url)
//     //     .headers(headers)
//     //     .send()
//     //     .await
//     //     .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

//     let response = client
//         .get(url)
//         .bearer_auth(api_key) // This replaces the manual HeaderMap logic
//         .send()
//         .await
//         .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

//     let paystack_res: VerifyResponse = response
//         .json()
//         .await
//         .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

//     println!("Data {:#?}", paystack_res);
//     Ok(Json(VerifyResponse {
//         status: paystack_res.status,
//         message: paystack_res.message,
//         data: VerifyResponseData {
//             status: paystack_res.data.status,
//             amount: paystack_res.data.amount,
//             reference: paystack_res.data.reference,
//             gateway_response: paystack_res.data.gateway_response,
//         },
//     }))
// }

// async fn verify_payment(Path(reference): Path<String>)
// // -> Json<VerifyResponse>
// {
//     let api_key = std::env::var("PAYSTACK_API_KEY").unwrap();
//     let url_main = std::env::var("PAYSTACK_TRANS_VERIFY_URL").unwrap();

//     let url = format!("{}{}", url_main, reference);
//     let client = reqwest::Client::new();

//     let res: VerifyResponse = client
//         .get(url)
//         .bearer_auth(api_key)
//         .send()
//         .await
//         .unwrap()
//         .json() // Json(VerifyResponse {
//         //     status: res.status,
//         //     message: res.message,
//         //     data: VerifyResponseData {
//         //         status: res.data.status,
//         //         amount: res.data.amount,
//         //         reference: res.data.reference,
//         //         gateway_response: res.data.gateway_response,
//         //     },
//         // })
//         .await
//         .unwrap();
//     println!("Data {:#?}", res);

//     // Json(VerifyResponse {
//     //     status: res.status,
//     //     message: res.message,
//     //     data: VerifyResponseData {
//     //         status: res.data.status,
//     //         amount: res.data.amount,
//     //         reference: res.data.reference,
//     //         gateway_response: res.data.gateway_response,
//     //     },
//     // })

//     // Check if the transaction itself was successful
//     // Ok(res.data.status == "success")
// }

//* ============================================================================
//* WEBHOOK: function to handle incoming webhook events from Paystack, verify their         authenticity, and process them accordingly.
//* ============================================================================

type HmacSha512 = Hmac<Sha512>;

async fn paystack_webhook(headers: HeaderMap, body: Bytes) -> StatusCode {
    let api_key = std::env::var("PAYSTACK_API_KEY").unwrap();

    // 1. Get the signature from headers
    let signature = match headers.get("x-paystack-signature") {
        Some(sig) => sig.to_str().unwrap_or(""),
        None => return StatusCode::BAD_REQUEST,
    };

    let mut mac =
        HmacSha512::new_from_slice(api_key.as_bytes()).expect("HMAC can take key of any size");
    mac.update(&body);

    let result = mac.finalize();
    let expected_signature = encode(result.into_bytes());

    if expected_signature != signature {
        return StatusCode::UNAUTHORIZED;
    }

    if let Ok(payload) = serde_json::from_slice::<serde_json::Value>(&body) {
        if payload["event"] == "charge.success" {
            let reference = payload["data"]["reference"].as_str().unwrap_or_default();
            println!("Payment Successful for ref: {}", reference);
        }
    }

    StatusCode::OK
}

//* ============================================================================
//* MAIN: Building the Router and Serving
//* ============================================================================

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/echo", post(echo))
        .route("/initialize", post(initialize_payment))
        .route(
            "/initialize/redirect/{email}/{amount}",
            post(initialize_payment_redirect),
        )
        // .route("/verify/{reference}", get(verify_payment))
        .route("/webhook", post(paystack_webhook))
        .fallback(not_found);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001")
        .await
        .expect("Failed to bind to port 3000");

    println!("============================================");
    println!("Server running on http://localhost:3001");
    println!("============================================");

    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}

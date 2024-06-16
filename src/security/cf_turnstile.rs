use crate::constants::constants;
use actix_web::HttpRequest;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid;

/// https://developers.cloudflare.com/turnstile/get-started/server-side-validation/#accepted-parameters
#[derive(Serialize, Deserialize, Debug)]
struct SiteVerifyRequest {
    secret: String,            // secret key for the API
    response: String,          // response token from the client
    remote_ip: Option<String>, // IP address of the client
    idempotency_key: Option<uuid::Uuid>,
}

// https://developers.cloudflare.com/turnstile/get-started/server-side-validation/#error-codes
#[derive(Serialize, Deserialize, Debug, Error)]
enum ErrorCodes {
    #[serde(rename = "missing-input-secret")]
    #[error("The secret parameter was not passed.")]
    MissingInputSecret,

    #[serde(rename = "invalid-input-secret")]
    #[error("The secret parameter was invalid or did not exist.")]
    InvalidInputSecret,

    #[serde(rename = "missing-input-response")]
    #[error("The response parameter was not passed.")]
    MissingInputResponse,

    #[serde(rename = "invalid-input-response")]
    #[error("The response parameter is invalid or has expired.")]
    InvalidInputResponse,

    #[serde(rename = "invalid-widget-id")]
    #[error(
        "The widget ID extracted from the parsed site secret key was invalid or did not exist."
    )]
    InvalidWidgetId,

    #[serde(rename = "invalid-parsed-secret")]
    #[error("The secret extracted from the parsed site secret key was invalid.")]
    InvalidParsedSecret,

    #[serde(rename = "bad-request")]
    #[error("The request was rejected because it was malformed.")]
    BadRequest,

    #[serde(rename = "timeout-or-duplicate")]
    #[error("The response parameter has already been validated before.")]
    TimeoutOrDuplicate,

    #[serde(rename = "internal-error")]
    #[error(
        "An internal error happened while validating the response. The request can be retried."
    )]
    InternalError,
}

#[derive(Serialize, Deserialize, Debug)]
struct SiteVerifyResponse {
    success: bool,
    challenge_ts: Option<String>,
    hostname: Option<String>,
    #[serde(rename = "error-codes")]
    error_codes: Vec<ErrorCodes>,
    action: Option<String>,
    cdata: Option<String>,
}

pub fn get_ip_addr(req: &HttpRequest) -> Option<String> {
    let cloudflare_proxy = req.headers().get("cf-connecting-ip");
    match cloudflare_proxy {
        Some(ip) => Some(ip.to_str().unwrap().to_string()),
        None => match req.connection_info().realip_remote_addr() {
            Some(ip) => Some(ip.to_string()),
            None => {
                log::error!("could not get ip address");
                None
            }
        },
    }
}

pub async fn verify_request(req: &HttpRequest, cf_response: &str) -> bool {
    let req_ip = get_ip_addr(req);
    let client = Client::new();
    let request_values = SiteVerifyRequest {
        secret: std::env::var(constants::CF_TURNSTILE_SECRET_KEY).unwrap(),
        response: cf_response.to_string(),
        remote_ip: req_ip,
        idempotency_key: Some(uuid::Uuid::new_v4()),
    };

    let response = client
        .post("https://challenges.cloudflare.com/turnstile/v0/siteverify")
        .json(&request_values)
        .send()
        .await;
    match response {
        Ok(response) => {
            if !response.status().is_success() {
                log::error!("Failed to verify response: {}", response.status());
                return false;
            }
            match response.json::<SiteVerifyResponse>().await {
                Ok(json_response) => {
                    if json_response.success {
                        log::info!("Successfully verified response");
                        true
                    } else {
                        log::error!("Failed to verify response: {:?}", json_response);
                        false
                    }
                }
                Err(e) => {
                    log::error!("Failed to deserialise JSON response: {}", e);
                    false
                }
            }
        }
        Err(e) => {
            log::error!("Failed to verify response: {}", e);
            false
        }
    }
}

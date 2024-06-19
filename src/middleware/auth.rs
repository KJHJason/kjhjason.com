use std::future::{ready, Ready};
use std::rc::Rc;

use actix_web::body::{BoxBody, EitherBody};
use actix_web::cookie::Cookie;
use actix_web::http::{header::ContentType, Method, StatusCode};
use actix_web::web::Data;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use askama::Template;
use bson::oid::ObjectId;
use futures_util::future::LocalBoxFuture;
use hmac_serialiser_rs::{HmacSigner, SignerLogic};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::constants::constants;
use crate::templates::error::ErrorTemplate;
use crate::utils::security::{get_default_key_info, get_default_salt};

macro_rules! auth_failed {
    ($req:expr, $status:expr) => {
        let html = ErrorTemplate {
            common: crate::utils::security::extract_for_template($req.request()),
            status: $status.as_u16(),
            message: $status.canonical_reason().unwrap_or("Not Found"),
        }
        .render()
        .unwrap();
        let mut auth_cookie = Cookie::build(constants::AUTH_COOKIE_NAME, "")
            .path("/")
            .domain(constants::get_domain())
            .http_only(true)
            .finish();
        auth_cookie.make_removal();
        let res = HttpResponse::build($status)
            .cookie(auth_cookie)
            .content_type(ContentType::html())
            .body(html);
        return Ok($req.into_response(res).map_into_right_body());
    };
}

// pub fn get_user_claim(req: &HttpRequest) -> Result<UserClaim, Error> {
//     match req.extensions().get::<UserClaim>() {
//         Some(user_claim) => Ok(user_claim.clone()),
//         None => {
//             // Shouldn't happen if the middleware is working correctly
//             log::error!("UserClaim not found in request extensions");
//             Err(actix_web::error::ErrorNotFound(""))
//         }
//     }
// }

#[derive(Serialize, Deserialize, Clone)]
pub struct UserClaim {
    #[serde(rename = "_id")]
    #[serde(serialize_with = "bson::serde_helpers::serialize_object_id_as_hex_string")]
    pub session_id: ObjectId,
    #[serde(serialize_with = "bson::serde_helpers::serialize_object_id_as_hex_string")]
    pub user_id: ObjectId,
}

impl hmac_serialiser_rs::Data for UserClaim {
    fn get_exp(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        None
    }
}

pub fn create_user_claim(user_id: ObjectId, id: ObjectId) -> UserClaim {
    UserClaim {
        user_id,
        session_id: id,
    }
}

pub fn get_default_auth_signer() -> HmacSigner {
    HmacSigner::new(
        get_default_key_info(get_default_salt(), vec![]),
        hmac_serialiser_rs::algorithm::Algorithm::SHA512,
        hmac_serialiser_rs::Encoder::UrlSafeNoPadding,
    )
}

macro_rules! init_auth_signer {
    () => {
        Lazy::new(|| get_default_auth_signer())
    };
}

// pub fn unsign_payload(payload: &str) -> Result<UserClaim, hmac_serialiser_rs::errors::Error> {
//     static AUTH_SIGNER: Lazy<HmacSigner> = init_auth_signer!();
//     AUTH_SIGNER.unsign::<UserClaim>(payload)
// }

pub fn sign_payload(user_claim: &UserClaim) -> String {
    static AUTH_SIGNER: Lazy<HmacSigner> = init_auth_signer!();
    AUTH_SIGNER.sign(user_claim)
}

#[derive(Clone)]
struct UserAuth {
    csrf_signer: HmacSigner,
    cookie_name: String,
    whitelist: Vec<(Method, String)>,
    whitelist_regex: Vec<(Method, regex::Regex)>,
}

impl UserAuth {
    pub fn new(
        csrf_signer: HmacSigner,
        cookie_name: String,
        whitelist: Vec<(Method, String)>,
        whitelist_regex: Vec<(Method, regex::Regex)>,
    ) -> Self {
        Self {
            csrf_signer,
            cookie_name,
            whitelist,
            whitelist_regex,
        }
    }
    pub fn requires_auth(&self, req: &ServiceRequest) -> bool {
        crate::utils::security::is_protected(&self.whitelist, &self.whitelist_regex, req)
    }
}

#[derive(Clone)]
pub struct AuthMiddleware {
    inner: UserAuth,
}

impl AuthMiddleware {
    pub fn new(
        signer: Option<HmacSigner>,
        cookie_name: &str,
        whitelist: Vec<(Method, String)>,
        whitelist_regex: Vec<(Method, regex::Regex)>,
    ) -> Self {
        let signer = signer.unwrap_or_else(|| get_default_auth_signer());
        let inner = UserAuth::new(signer, cookie_name.to_string(), whitelist, whitelist_regex);
        Self { inner }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B, BoxBody>>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service: Rc::new(service),
            inner: Rc::new(self.inner.clone()),
        }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
    inner: Rc<UserAuth>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B, BoxBody>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Request method of OPTIONS is used for Cors preflight requests
        if req.method() == Method::OPTIONS || !self.inner.requires_auth(&req) {
            let fut = self.service.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res.map_into_left_body())
            });
        }

        let auth_cookie = match req.cookie(&self.inner.cookie_name) {
            Some(auth_cookie) => auth_cookie,
            None => {
                return Box::pin(async {
                    log::warn!("No auth cookie found");
                    auth_failed!(req, StatusCode::NOT_FOUND);
                });
            }
        };

        let user_claim = match self
            .inner
            .csrf_signer
            .unsign::<UserClaim>(&auth_cookie.value())
        {
            Ok(user_claim) => user_claim,
            Err(e) => {
                return match e {
                    hmac_serialiser_rs::errors::Error::TokenExpired => Box::pin(async move {
                        log::warn!("token expired");
                        auth_failed!(req, StatusCode::UNAUTHORIZED);
                    }),
                    _ => Box::pin(async move {
                        log::warn!("Failed to unsign payload: {:?}", e);
                        auth_failed!(req, StatusCode::NOT_FOUND);
                    }),
                }
            }
        };

        // Since the mongodb client is using an Arc, it is very cheap to clone it.
        // Additionally, in the app_data, we are using a Data wrapper which uses an Arc internally.
        // Hence, performance wise, it shouldn't be a problem!
        // Ref: https://stackoverflow.com/questions/40984932/what-happens-when-an-arc-is-cloned
        // Doc: https://docs.rs/mongodb/latest/mongodb/struct.Client.html
        let client = req
            .app_data::<Data<crate::database::db::DbClient>>()
            .unwrap()
            .clone();

        let service = Rc::clone(&self.service);
        Box::pin(async move {
            let session = match client.get_session_by_id(&user_claim.session_id).await {
                Ok(session) => session,
                Err(_) => {
                    log::warn!("Session not found");
                    auth_failed!(req, StatusCode::NOT_FOUND);
                }
            };
            if session.is_expired() {
                log::warn!("Session expired");
                auth_failed!(req, StatusCode::UNAUTHORIZED);
            }
            if session.user_id != user_claim.user_id {
                log::warn!("Invalid session as user_id does not match the session's user_id");
                auth_failed!(req, StatusCode::NOT_FOUND);
            }

            let fut = service.call(req).await?;
            Ok(fut.map_into_left_body())
        })
    }
}

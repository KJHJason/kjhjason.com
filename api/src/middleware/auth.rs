use actix_web::http::Method;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use bson::oid::ObjectId;
use futures_util::future::LocalBoxFuture;
use hmac_serialiser_rs::{HmacSigner, SignerLogic};
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};
use std::rc::Rc;

macro_rules! auth_failed {
    ($msg:expr) => {
        log::warn!("{}", $msg);
        return Err(actix_web::error::ErrorNotFound("not found"));
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
    #[serde(serialize_with = "bson::serde_helpers::serialize_object_id_as_hex_string")]
    pub id: ObjectId,

    #[serde(with = "crate::utils::datetime::rfc3339")]
    pub exp: chrono::DateTime<chrono::Utc>,
}

pub fn create_user_claim(id: ObjectId, exp_sec: i64) -> UserClaim {
    UserClaim {
        id,
        exp: chrono::Utc::now() + chrono::Duration::seconds(exp_sec),
    }
}

impl hmac_serialiser_rs::Data for UserClaim {
    fn get_exp(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        Some(self.exp)
    }
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
    pub fn requires_auth(&self, method: &Method, path: &str) -> bool {
        for (allowed_method, allowed_path) in &self.whitelist {
            if allowed_method == method && allowed_path == path {
                return false;
            }
        }
        for (allowed_method, allowed_path) in &self.whitelist_regex {
            if allowed_method == method && allowed_path.is_match(path) {
                return false;
            }
        }
        true
    }
    pub fn get_auth_cookie(&self, req: &ServiceRequest) -> Result<String, Error> {
        match req.cookie(&self.cookie_name) {
            Some(cookie) => Ok(cookie.value().to_string()),
            None => Err(Error::from(actix_web::error::ErrorNotFound(""))),
        }
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
        let signer = signer.unwrap_or_else(|| crate::utils::security::get_auth_signer());
        let inner = UserAuth::new(signer, cookie_name.to_string(), whitelist, whitelist_regex);
        Self { inner }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service,
            inner: Rc::new(self.inner.clone()),
        }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
    inner: Rc<UserAuth>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        if self.inner.requires_auth(req.method(), req.path()) {
            let auth_cookie = match self.inner.get_auth_cookie(&req) {
                Ok(auth_cookie) => auth_cookie,
                Err(e) => {
                    return Box::pin(async move {
                        auth_failed!(format!("auth cookie error: {}", e));
                    });
                }
            };

            match self.inner.csrf_signer.unsign::<UserClaim>(&auth_cookie) {
                Ok(user_claim) => {
                    req.extensions_mut().insert(user_claim);
                }
                Err(_) => {
                    return Box::pin(async move {
                        auth_failed!(format!("invalid auth cookie, {}", auth_cookie));
                    });
                }
            }
        }

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}

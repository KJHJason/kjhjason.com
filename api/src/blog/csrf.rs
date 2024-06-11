use crate::constants::constants;
use crate::middleware::csrf;
use crate::model::base_error::Error;
use actix_web::{get, web, HttpMessage, HttpRequest};
use serde::Deserialize;

#[derive(Deserialize)]
struct Redirect {
    redirect: Option<String>,
}

#[get("/csrf-token")]
async fn get_csrf_token(
    req: HttpRequest,
    redirect: web::Query<Redirect>,
) -> Result<web::Redirect, Error> {
    // get redirect GET param
    let redirect = match &redirect.redirect {
        Some(redirect) => {
            // only allow relative redirects
            if redirect.starts_with('/') {
                redirect.to_string()
            } else {
                "/".to_string()
            }
        }
        None => "/".to_string(),
    };
    let full_redirect = format!(
        "{}://{}{}/{}",
        constants::get_scheme(),
        constants::get_domain(),
        constants::get_port(),
        redirect,
    );

    match req.cookie(constants::CSRF_COOKIE_NAME) {
        Some(_) => {
            // redirect user to the specified location
            Ok(web::Redirect::to(full_redirect).see_other())
        }
        None => {
            // either the middleware has yet to add the CSRF token in the response or the token is missing which shouldn't happen
            req.extensions()
                .get::<csrf::HasCsrfCookie>()
                .ok_or_else(|| Error::new("CSRF token not found".to_string()))?;
            Ok(web::Redirect::to(full_redirect).see_other())
        }
    }
}

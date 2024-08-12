use crate::constants;
use crate::errors::base::Error;
use crate::middleware::csrf;
use crate::utils::redirect;

use actix_web::{get, web, HttpMessage, HttpRequest};

#[get("/api/csrf-token")]
async fn get_csrf_token(
    req: HttpRequest,
    redirect: web::Query<redirect::RedirectParams>,
) -> Result<web::Redirect, Error> {
    let full_redirect = redirect::get_redirect_url(&redirect);
    match req.cookie(constants::CSRF_COOKIE_NAME) {
        Some(_) => {
            // redirect user to the specified location
            Ok(web::Redirect::to(full_redirect).see_other())
        }
        None => {
            // either the middleware has yet to add the CSRF token in the response or the token is missing which shouldn't happen
            req.extensions()
                .get::<csrf::CsrfValue>()
                .ok_or_else(|| Error::new("CSRF token not found".to_string()))?;
            Ok(web::Redirect::to(full_redirect).see_other())
        }
    }
}

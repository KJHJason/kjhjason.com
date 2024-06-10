use crate::constants::constants;
use crate::model::{base_error::Error, csrf::CsrfResponse};
use actix_web::{get, HttpRequest, HttpResponse};

#[get("/csrf-token")]
async fn get_csrf_token(req: HttpRequest) -> HttpResponse {
    let csrf_token = match req.cookie(constants::CSRF_HEADER_NAME) {
        Some(cookie) => cookie.value().to_string(),
        None => {
            // shouldn't happen due to the CSRF middleware adding the csrf token automatically
            return HttpResponse::InternalServerError()
                .json(Error::new("CSRF token cookie not found".to_string()));
        }
    };
    HttpResponse::Ok().json(CsrfResponse::new(csrf_token))
}

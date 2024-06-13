use actix_web::web;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RedirectParams {
    redirect: Option<String>,
}

pub fn get_redirect_url(redirect: &web::Query<RedirectParams>) -> String {
    let redirect = match &redirect.redirect {
        Some(redirect) => {
            // only allow relative redirects
            if redirect.starts_with("http") {
                "".to_string()
            } else {
                redirect.to_string()
            }
        }
        None => "".to_string(),
    };

    format!("/{}", redirect)
}

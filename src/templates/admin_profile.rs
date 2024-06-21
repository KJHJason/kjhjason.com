use askama::Template;

#[derive(Template)]
#[template(path = "components/enable_2fa.html")]
pub struct Enable2FA {
    pub csrf_header_json: String,
}

#[derive(Template)]
#[template(path = "components/disable_2fa.html")]
pub struct Disable2FA {
    pub csrf_header_json: String,
}

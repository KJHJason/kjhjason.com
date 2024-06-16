use askama::Template;

#[derive(Template)]
#[template(path = "components/alert_success.html")]
pub struct SucessAlert<'a> {
    pub msg: &'a str,
}

#[derive(Template)]
#[template(path = "components/alert_error.html")]
pub struct ErrAlert<'a> {
    pub err: &'a str,
}

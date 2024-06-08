use serde::Serialize;

#[derive(Serialize)]
pub struct Msg {
    message: String,
}

impl Msg {
    pub fn new(msg: String) -> Msg {
        Msg { message: msg }
    }
}

use askama::Template;

#[derive(Template)]
#[template(path = "components/guest_items.html")]
pub struct GuestItems;

use crate::database::db;
use crate::model::blog::BlogIdentifier;
use crate::templates::admin::{EditBlog, NewBlog};
use crate::templates::error::ErrorTemplate;
use crate::utils::security::extract_for_template;
use crate::utils::validations::get_id_from_path;
use actix_web::http::header::ContentType;
use actix_web::web::{Data, Path};
use actix_web::{get, HttpRequest, HttpResponse, Responder};
use askama::Template;

#[get("/admin/new/blog")]
async fn new_blog(req: HttpRequest) -> impl Responder {
    NewBlog {
        common: extract_for_template(&req),
        post_blog_btn_txt: "Publish Blog",
    }
}

#[get("/admin/blogs/{id}/edit")]
async fn edit_blog(
    client: Data<db::DbClient>,
    req: HttpRequest,
    blog_identifier: Path<BlogIdentifier>,
) -> HttpResponse {
    let blog_id = match get_id_from_path(&req, blog_identifier) {
        Ok(blog_id) => blog_id,
        Err(response) => return response,
    };
    let blog = client.into_inner().get_blog_post(&blog_id, None).await;
    let blog = match blog {
        Ok(blog) => blog,
        Err(_) => {
            let html = ErrorTemplate {
                common: extract_for_template(&req),
                status: 500,
                message: "Failed to get blog post",
            }
            .render()
            .unwrap();
            return HttpResponse::InternalServerError()
                .content_type(ContentType::html())
                .body(html);
        }
    };

    let html = EditBlog {
        common: extract_for_template(&req),
        id: &blog_id.to_hex(),
        title: &blog.title,
        content: &blog.content,
        public: blog.is_public,
        tags: &blog.tags.join(", "),
        post_blog_btn_txt: "Update Blog",
    }
    .render()
    .unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html)
}

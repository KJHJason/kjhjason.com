use crate::database::db;
use crate::middleware::auth::get_user_claim;
use crate::models::blog_identifier::BlogIdentifier;
use crate::templates::admin::{EditBlog, NewBlog, Profile};
use crate::templates::error::ErrorTemplate;
use crate::utils::{
    html::render_template, security::extract_for_template, validations::get_id_from_path,
};
use actix_web::http::StatusCode;
use actix_web::web::{Data, Path};
use actix_web::{get, HttpRequest, HttpResponse};
use bson::doc;
use mongodb::options::FindOneOptions;

#[get("/admin/new/blog")]
async fn new_blog(req: HttpRequest) -> HttpResponse {
    let template = NewBlog {
        common: extract_for_template(&req),
        post_blog_btn_txt: "Publish Blog",
    };
    render_template(template, StatusCode::OK)
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
            let template = ErrorTemplate {
                common: extract_for_template(&req),
                status: 500,
                message: "Failed to get blog post",
            };
            return render_template(template, StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let template = EditBlog {
        common: extract_for_template(&req),
        id: &blog_id.to_hex(),
        title: &blog.title,
        seo_desc: &blog.seo_desc,
        content: &blog.content,
        public: blog.is_public,
        tags: &blog.tags.join(", "),
        post_blog_btn_txt: "Update Blog",
    };
    // since the minification will not preserve the whitespace in the content
    render_template(template, StatusCode::OK)
}

#[get("/admin/profile")]
async fn profile(client: Data<db::DbClient>, req: HttpRequest) -> HttpResponse {
    let user_info = get_user_claim(&req);
    let options = FindOneOptions::builder()
        .projection(doc! {"totp_secret": 1})
        .build();
    let user = client
        .get_projected_user_by_id(&user_info.user_id, Some(options))
        .await
        .map_err(|_| {
            let template = ErrorTemplate {
                common: extract_for_template(&req),
                status: 500,
                message:
                    "Something went wrong while fetching your profile. Please try again later.",
            };
            render_template(template, StatusCode::INTERNAL_SERVER_ERROR)
        });
    if user.is_err() {
        return user.unwrap_err();
    }

    let template = Profile {
        common: extract_for_template(&req),
        has_2fa: !user.unwrap().totp_secret.unwrap_or_default().is_empty(),
    };
    render_template(template, StatusCode::OK)
}

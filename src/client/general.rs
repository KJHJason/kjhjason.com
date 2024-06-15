use crate::client::templates::general::{
    Blog, BlogPost, BlogPostInfo, Experiences, Index, Projects, Skills,
};
use crate::database::db;
use crate::middleware::errors::ErrorTemplate;
use crate::model::blog::BlogIdentifier;
use crate::utils::security::extract_for_template;
use crate::utils::validations::validate_id;
use actix_web::http::header::ContentType;
use actix_web::web::Data;
use actix_web::{get, web::Path, HttpRequest, HttpResponse, Responder};
use askama::Template;
use futures_util::TryStreamExt;
use mongodb::bson::doc;
use mongodb::options::{FindOneAndUpdateOptions, ReturnDocument};

#[get("/")]
async fn index(req: HttpRequest) -> impl Responder {
    Index {
        common: extract_for_template(&req),
    }
}

#[get("/experiences")]
async fn experiences(req: HttpRequest) -> impl Responder {
    Experiences {
        common: extract_for_template(&req),
    }
}

#[get("/projects")]
async fn projects(req: HttpRequest) -> impl Responder {
    Projects {
        common: extract_for_template(&req),
    }
}

#[get("/skills")]
async fn skills(req: HttpRequest) -> impl Responder {
    Skills {
        common: extract_for_template(&req),
    }
}

#[get("/blog")]
async fn blog(client: Data<db::DbClient>, req: HttpRequest) -> HttpResponse {
    // get all blogs
    let mut blogs_cursor = match client.get_blog_collection().find(None, None).await {
        Ok(blogs) => blogs,
        Err(_) => {
            let html = ErrorTemplate {
                common: extract_for_template(&req),
                status: 500,
                message: "Failed to get blog posts",
            }
            .render()
            .unwrap();
            return HttpResponse::InternalServerError()
                .content_type(ContentType::html())
                .body(html);
        }
    };

    // could pre-allocate the vector size but not worth
    // the extra connection to the db which could be slower
    let mut blogs = Vec::new();
    loop {
        // do a loop this way for logging any errors compared to
        // while let Ok(Some(blog_post)) = blogs_cursor.try_next().await
        match blogs_cursor.try_next().await {
            Ok(Some(blog_post)) => {
                let blog_info = BlogPostInfo {
                    id: blog_post.get_id_string(),
                    title: blog_post.get_title().to_string(),
                    date: blog_post.get_date_string(),
                    views: blog_post.get_views(),
                };
                blogs.push(blog_info);
            }
            Ok(None) => break,
            Err(e) => {
                log::error!("Failed to get blog post: {}", e);
                let html = ErrorTemplate {
                    common: extract_for_template(&req),
                    status: 500,
                    message: "Failed to get blog posts",
                }
                .render()
                .unwrap();
                return HttpResponse::InternalServerError()
                    .content_type(ContentType::html())
                    .body(html);
            }
        }
    }

    let html = Blog {
        common: extract_for_template(&req),
        blogs,
    }
    .render()
    .unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html)
}

#[get("/blog/{id}")]
async fn blog_id(
    client: Data<db::DbClient>,
    req: HttpRequest,
    blog_id: Path<BlogIdentifier>,
) -> HttpResponse {
    let blog_id = match validate_id(&blog_id.into_inner().get_id()) {
        Ok(blog_id) => blog_id,
        Err(_) => {
            let html = ErrorTemplate {
                common: extract_for_template(&req),
                status: 400,
                message: "Invalid blog post ID",
            }
            .render()
            .unwrap();
            return HttpResponse::BadRequest()
                .content_type(ContentType::html())
                .body(html);
        }
    };
    let query = doc! { "_id": blog_id };
    let update = doc! { "$inc": { "views": 1 } };
    let options = FindOneAndUpdateOptions::builder()
        .return_document(ReturnDocument::After)
        .build();

    let blog_post = client
        .get_blog_collection()
        .find_one_and_update(query, update, Some(options))
        .await;

    match blog_post {
        Ok(Some(blog_post)) => {
            let html = BlogPost {
                common: extract_for_template(&req),
                title: blog_post.get_title(),
                date: &blog_post.get_date_string(),
                readable_date: &blog_post.get_readable_date_diff(),
                views: blog_post.get_views(),
                content: &blog_post.get_html_content(),
            }
            .render()
            .unwrap();
            HttpResponse::Ok()
                .content_type(ContentType::html())
                .body(html)
        }
        Ok(None) => {
            let html = ErrorTemplate {
                common: extract_for_template(&req),
                status: 404,
                message: "Blog post not found",
            }
            .render()
            .unwrap();
            HttpResponse::NotFound()
                .content_type(ContentType::html())
                .body(html)
        }
        Err(_) => {
            let html = ErrorTemplate {
                common: extract_for_template(&req),
                status: 500,
                message: "Failed to get blog post",
            }
            .render()
            .unwrap();
            HttpResponse::InternalServerError()
                .content_type(ContentType::html())
                .body(html)
        }
    }
}

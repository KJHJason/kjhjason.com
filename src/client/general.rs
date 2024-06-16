use crate::database::db;
use crate::model::blog::BlogIdentifier;
use crate::templates::error::ErrorTemplate;
use crate::templates::general::{
    Blog, BlogPost, BlogPostInfo, Experiences, Index, Projects, Skills,
};
use crate::utils::security::extract_for_template;
use crate::utils::validations::get_id_from_path;
use actix_web::http::header::ContentType;
use actix_web::web::Data;
use actix_web::{get, web::Path, HttpRequest, HttpResponse, Responder};
use askama::Template;
use futures_util::TryStreamExt;
use mongodb::bson::doc;
use mongodb::options::{FindOneAndUpdateOptions, FindOptions, ReturnDocument};

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

#[get("/blogs")]
async fn blogs(client: Data<db::DbClient>, req: HttpRequest) -> HttpResponse {
    // get all blogs
    let find_options = FindOptions::builder()
        .sort(doc! { "_id": -1 }) // get by newest first
        .build();
    let mut blogs_cursor = match client.get_blog_collection().find(None, find_options).await {
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

    let common = extract_for_template(&req);

    // could pre-allocate the vector size but not worth
    // the extra connection to the db which could be slower
    let mut blogs = Vec::new();
    loop {
        // do a loop this way for logging any errors compared to
        // while let Ok(Some(blog_post)) = blogs_cursor.try_next().await
        match blogs_cursor.try_next().await {
            Ok(Some(blog_post)) => {
                if !blog_post.is_public && !common.is_logged_in {
                    continue;
                }
                let id = blog_post.get_id_string();
                let date = blog_post.get_date_string();
                let blog_info = BlogPostInfo {
                    id,
                    title: blog_post.title,
                    date,
                    views: blog_post.views,
                    tags: blog_post.tags,
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

    let html = Blog { common, blogs }.render().unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html)
}

macro_rules! blog_not_found {
    ($req:expr) => {
        let html = ErrorTemplate {
            common: extract_for_template(&$req),
            status: 404,
            message: "Blog post not found",
        }
        .render()
        .unwrap();
        return HttpResponse::NotFound()
            .content_type(ContentType::html())
            .body(html);
    };
}

#[get("/blogs/{id}")]
async fn blog_id(
    client: Data<db::DbClient>,
    req: HttpRequest,
    blog_identifier: Path<BlogIdentifier>,
) -> HttpResponse {
    let blog_id = match get_id_from_path(&req, blog_identifier) {
        Ok(blog_id) => blog_id,
        Err(response) => return response,
    };
    let query = doc! { "_id": blog_id };
    let common = extract_for_template(&req);
    let blog_collection = client.get_blog_collection();
    let blog_post = if common.is_logged_in {
        blog_collection.find_one(query, None).await
    } else {
        let update = doc! { "$inc": { "views": 1 } };
        let options = FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After)
            .build();

        blog_collection
            .find_one_and_update(query, update, Some(options))
            .await
    };

    match blog_post {
        Ok(Some(blog_post)) => {
            if !blog_post.is_public && !common.is_logged_in {
                blog_not_found!(req);
            }

            let html = BlogPost {
                common,
                id: &blog_post.get_id_string(),
                title: &blog_post.title,
                date: &blog_post.get_date_string(),
                readable_date: &blog_post.get_readable_date_diff(),
                last_modified: &blog_post.get_last_modified_date_string(),
                views: blog_post.views,
                content: &blog_post.get_html_content(),
                public: blog_post.is_public,
                tags: &blog_post.tags,
            }
            .render()
            .unwrap();
            HttpResponse::Ok()
                .content_type(ContentType::html())
                .body(html)
        }
        Ok(None) => {
            blog_not_found!(req);
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

use crate::database::db;
use crate::models::blog;
use crate::models::blog_identifier::BlogIdentifier;
use crate::templates::error::ErrorTemplate;
use crate::templates::general::{
    Awards, BlogPost, BlogPostInfo, Blogs, Certificates, Experiences, Index, Projects, Skills,
};
use crate::utils::awards::get_awards;
use crate::utils::certificates::get_certificates;
use crate::utils::experiences::get_experiences;
use crate::utils::html::render_template;
use crate::utils::projects::get_projects;
use crate::utils::security::extract_for_template;
use crate::utils::skills::{
    get_backend, get_database, get_deployment, get_frontend, get_general, get_languages,
};
use crate::utils::validations::get_id_from_path;
use actix_web::http::StatusCode;
use actix_web::web::Data;
use actix_web::{get, web::Path, HttpRequest, HttpResponse};
use futures_util::TryStreamExt;
use mongodb::bson::doc;
use mongodb::options::{FindOneAndUpdateOptions, FindOptions, ReturnDocument};

#[get("/")]
async fn index(req: HttpRequest) -> HttpResponse {
    let template = Index {
        common: extract_for_template(&req),
    };
    render_template(template, StatusCode::OK)
}

#[get("/experiences")]
async fn experiences(req: HttpRequest) -> HttpResponse {
    let template = Experiences {
        common: extract_for_template(&req),
        experiences: get_experiences(),
    };
    render_template(template, StatusCode::OK)
}

#[get("/projects")]
async fn projects(req: HttpRequest) -> HttpResponse {
    let template = Projects {
        common: extract_for_template(&req),
        projects: get_projects(),
    };
    render_template(template, StatusCode::OK)
}

#[get("/skills")]
async fn skills(req: HttpRequest) -> HttpResponse {
    let template = Skills {
        common: extract_for_template(&req),
        languages: get_languages(),
        backend: get_backend(),
        frontend: get_frontend(),
        database: get_database(),
        deployment: get_deployment(),
        general: get_general(),
    };
    render_template(template, StatusCode::OK)
}

#[get("/certificates")]
async fn certificates(req: HttpRequest) -> HttpResponse {
    let template = Certificates {
        common: extract_for_template(&req),
        certificates: get_certificates(),
    };
    render_template(template, StatusCode::OK)
}

#[get("/awards")]
async fn awards(req: HttpRequest) -> HttpResponse {
    let template = Awards {
        common: extract_for_template(&req),
        awards: get_awards(),
    };
    render_template(template, StatusCode::OK)
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
            let template = ErrorTemplate {
                common: extract_for_template(&req),
                status: 500,
                message: "Failed to get blog posts",
            };
            return render_template(template, StatusCode::INTERNAL_SERVER_ERROR);
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
                let template = ErrorTemplate {
                    common: extract_for_template(&req),
                    status: 500,
                    message: "Failed to get blog posts",
                };
                return render_template(template, StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    }

    let template = Blogs { common, blogs };
    render_template(template, StatusCode::OK)
}

macro_rules! blog_not_found {
    ($req:expr) => {
        let template = ErrorTemplate {
            common: extract_for_template(&$req),
            status: 404,
            message: "Blog post not found",
        };
        return render_template(template, StatusCode::NOT_FOUND);
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
        let update = doc! {"$inc": {blog::VIEWS_KEY: 1}};
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

            let template = BlogPost {
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
            };
            render_template(template, StatusCode::OK)
        }
        Ok(None) => {
            blog_not_found!(req);
        }
        Err(_) => {
            let template = ErrorTemplate {
                common: extract_for_template(&req),
                status: 500,
                message: "Failed to get blog post",
            };
            render_template(template, StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

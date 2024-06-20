pub mod file_utils {
    use crate::constants::constants;
    use crate::errors::blog::BlogError;
    use crate::models::file_info::FileInfo;
    use crate::utils::storage;
    use aws_sdk_s3 as s3;

    #[inline]
    fn change_obj_prefix(obj: &str, blog_id: &str, old_prefix: &str, new_prefix: &str) -> String {
        if !obj.starts_with(old_prefix) {
            log::error!("Object doesn't start with the old prefix");
            return obj.to_string();
        }
        let obj_without_prefix = &obj[old_prefix.len()..];
        format!("{}/{}{}", new_prefix, blog_id, obj_without_prefix)
    }

    macro_rules! delete_blob {
        ($client:expr, $file_url:expr) => {
            let (bucket, obj_name) = storage::extract_bucket_and_blob_from_url($file_url);
            if bucket.is_empty() || obj_name.is_empty() {
                return Err(BlogError::InternalServerError);
            }
            if !storage::delete_blob($client, &bucket, &obj_name).await {
                return Err(BlogError::InternalServerError);
            }
        };
    }

    macro_rules! move_blob {
        ($client:expr, $source_bucket:expr, $source_name:expr, $destination_bucket:expr, $destination_name:expr) => {
            if !storage::copy_blob(
                $client,
                $source_bucket,
                $source_name,
                $destination_bucket,
                $destination_name,
            )
            .await
            {
                return Err(BlogError::FileUploadError);
            }
        };
    }

    macro_rules! upload_blob {
        ($client:expr, $bucket:expr, $name:expr, $data:expr) => {
            if !storage::upload_blob($client, $bucket, $name, $data).await {
                return Err(BlogError::FileUploadError);
            }
        };
    }

    pub async fn process_file_logic(
        blog_id: &str,
        file: &mut FileInfo,
        content: &mut String,
        s3_client: &s3::Client,
    ) -> Result<(), BlogError> {
        if file.url.is_empty() {
            return Err(BlogError::FileIsEmpty);
        }

        let signed_url = match &file.signed_url {
            Some(url) => &url.clone(),
            None => {
                return Ok(());
            }
        };

        if !content.contains(signed_url) {
            // if the signed url is not in the content,
            // the user might have uploaded the file
            // but did not use it in the content.
            return Ok(());
        }

        let (bucket, obj_name) = storage::extract_bucket_and_blob_from_url(&file.url);
        if bucket.is_empty() || obj_name.is_empty() {
            return Err(BlogError::InternalServerError);
        }

        // replace the signed url with the actual url
        let obj_name_with_changed_prefix = &change_obj_prefix(
            &obj_name,
            blog_id,
            constants::TEMP_OBJ_PREFIX,
            constants::BLOG_OBJ_PREFIX,
        );
        let new_url = format!(
            "{}/{}",
            constants::PUBLIC_S3_URL,
            obj_name_with_changed_prefix,
        );
        let signed_url_idx = match content.find(signed_url) {
            Some(idx) => idx,
            None => {
                log::warn!("Signed url not found in content");
                return Ok(());
            }
        };
        content.replace_range(signed_url_idx..signed_url_idx + signed_url.len(), &new_url);
        file.signed_url = None;
        file.url = new_url;

        move_blob!(
            &s3_client,
            &bucket,
            &obj_name,
            constants::BUCKET,
            obj_name_with_changed_prefix
        );
        return Ok(());
    }

    /// Process the file and update the content with the new url instead of the signed url.
    ///
    /// If the file is not used in the content, it will not be processed.
    ///
    /// If the file is not found in the temp bucket, an error will be returned.
    ///
    /// Sample usage:
    /// ```rust
    /// use crate::utils::blog::file_utils;
    /// use crate::utils::blog::file_utils::process_file_logic;
    ///
    /// process_file!(blog_id, file, content, s3_client);
    /// ```
    macro_rules! process_file {
        ($blog_id:expr, $file:expr, $content:expr, $s3_client:expr) => {
            match process_file_logic($blog_id, $file, $content, $s3_client).await {
                Ok(_) => {}
                Err(err) => {
                    return Err(err);
                }
            }
        };
    }

    // thanks to https://stackoverflow.com/questions/26731243/how-do-i-use-a-macro-across-module-files
    pub(crate) use {delete_blob, move_blob, process_file, upload_blob};
}

pub mod publish_utils {
    use crate::database::db;
    use crate::errors::blog::BlogError;
    use crate::templates;
    use crate::utils::html::render_template;
    use crate::utils::validations::validate_id;
    use actix_web::http::StatusCode;
    use actix_web::web::Data;
    use actix_web::HttpResponse;
    use bson::doc;

    #[inline]
    pub async fn configure_blog_post_bool(
        client: Data<db::DbClient>,
        blog_id: &str,
        is_public: bool,
    ) -> Result<HttpResponse, BlogError> {
        let blog_id = validate_id(blog_id)?;
        let query = doc! { "_id": blog_id };
        let update = doc! { "$set": { "is_public": is_public } };
        let blog_col = client.into_inner().get_blog_collection();
        match blog_col.update_one(query, update, None).await {
            Ok(_) => {
                let response = if is_public {
                    render_template(templates::admin::Unlocked, StatusCode::OK)
                } else {
                    render_template(templates::admin::Locked, StatusCode::OK)
                };
                Ok(response)
            }
            Err(err) => {
                log::error!("Failed to publish api in database: {:?}", err);
                Err(BlogError::PublishBlogError)
            }
        }
    }
}

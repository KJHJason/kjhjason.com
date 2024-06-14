use crate::constants::constants;
use google_cloud_storage::client::Client;
use google_cloud_storage::http::objects::copy::CopyObjectRequest;
use google_cloud_storage::http::objects::delete::DeleteObjectRequest;
use google_cloud_storage::http::objects::upload::{Media, UploadObjectRequest, UploadType};
use google_cloud_storage::sign::{SignedURLMethod, SignedURLOptions};

pub async fn upload_blob(client: &Client, bucket: &str, obj_name: String, data: Vec<u8>) -> bool {
    let upload_type = UploadType::Simple(Media::new(obj_name));
    let uploaded = client
        .upload_object(
            &UploadObjectRequest {
                bucket: bucket.to_string(),
                ..Default::default()
            },
            data,
            &upload_type,
        )
        .await;
    match uploaded {
        Ok(_) => true,
        Err(e) => {
            log::error!("Error uploading object: {:?}", e);
            false
        }
    }
}

pub async fn get_signed_url(client: &Client, bucket: &str, obj_name: &str) -> String {
    client
        .signed_url(
            bucket,
            obj_name,
            None,
            None,
            SignedURLOptions {
                method: SignedURLMethod::GET,
                expires: constants::SIGNED_URL_MAX_AGE,
                ..Default::default()
            },
        )
        .await
        .unwrap_or_else(|e| {
            log::error!("Error generating signed URL: {:?}", e);
            "".to_string()
        })
}

pub async fn copy_blob(
    client: &Client,
    src_bucket: &str,
    src_obj_name: &str,
    dest_bucket: &str,
    dest_obj_name: &str,
) -> bool {
    match client
        .copy_object(&CopyObjectRequest {
            destination_bucket: dest_bucket.to_string(),
            destination_object: dest_obj_name.to_string(),
            source_bucket: src_bucket.to_string(),
            source_object: src_obj_name.to_string(),
            ..Default::default()
        })
        .await
    {
        Ok(_) => true,
        Err(e) => {
            log::error!("Error copying object: {:?}", e);
            false
        }
    }
}

pub async fn delete_blob(client: &Client, bucket: &str, name: &str) -> bool {
    match client
        .delete_object(&DeleteObjectRequest {
            bucket: bucket.to_string(),
            object: name.to_string(),
            ..Default::default()
        })
        .await
    {
        Ok(_) => true,
        Err(e) => {
            log::error!("Error deleting object: {:?}", e);
            false
        }
    }
}

pub fn extract_bucket_and_blob_from_url(url: &str) -> (String, String) {
    let url = url.trim_start_matches("https://storage.googleapis.com/");
    let mut parts = url.splitn(2, '/');
    let bucket = parts.next().unwrap_or_default();
    let blob = parts.next().unwrap_or_default();
    (bucket.to_string(), blob.to_string())
}

pub fn remove_file_from_md_content(content: &mut String, file_url: &str) -> String {
    let mut file_url = file_url.to_string();
    if let Some(index) = file_url.find('?') {
        // if ?X-Goog-Algorithm=GOOG4-RSA-SHA256&X-Goog-Credential=... is present, remove it
        file_url = file_url[..index].to_string();
    }

    let escaped_url = regex::escape(&file_url);
    let re = if file_url.ends_with(".mp4") {
        // use regex to remove <video src="url" controls> from content
        regex::Regex::new(&format!(r#"<video src="({})" controls>"#, escaped_url)).unwrap()
    } else {
        // use regex to remove ![alt](url) from content
        regex::Regex::new(&format!(r"!\[.*\]\({}\)", escaped_url)).unwrap()
    };
    re.replace_all(content, "").to_string()
}

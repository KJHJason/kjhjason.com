use crate::constants::constants;
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::{primitives::ByteStream, Client};
use once_cell::sync::Lazy;

pub async fn upload_blob(client: &Client, bucket: &str, obj_name: &str, data: Vec<u8>) -> bool {
    let body = ByteStream::from(data);
    client
        .put_object()
        .bucket(bucket)
        .key(obj_name)
        .body(body)
        .send()
        .await // Result<PutObjectOutput, SdkError<PutObjectError>>
        .map(|_| true)
        .unwrap_or_else(|e| {
            log::error!("Failed to upload blob: {:?}", e);
            false
        })
}

pub async fn copy_blob(
    client: &Client,
    src_bucket: &str,
    src_obj_name: &str,
    dest_bucket: &str,
    dest_obj_name: &str,
) -> bool {
    let source_obj_with_bucket: String = format!("{}/{}", src_bucket, src_obj_name,);
    client
        .copy_object()
        .copy_source(source_obj_with_bucket)
        .bucket(dest_bucket)
        .key(dest_obj_name)
        .send()
        .await // Result<CopyObjectOutput, SdkError<CopyObjectError>>
        .map(|_| true)
        .unwrap_or_else(|e| {
            log::error!("Failed to copy blob: {:?}", e);
            false
        })
}

pub async fn delete_blob(client: &Client, bucket: &str, name: &str) -> bool {
    // Result<DeleteObjectOutput, SdkError<DeleteObjectError>> {
    return client
        .delete_object()
        .bucket(bucket)
        .key(name)
        .send()
        .await
        .is_ok();
}

pub async fn get_signed_url(client: &Client, bucket: &str, obj_name: &str) -> String {
    let presigning_config = PresigningConfig::expires_in(constants::SIGNED_URL_MAX_AGE).unwrap();
    client
        .get_object()
        .bucket(bucket)
        .key(obj_name)
        .presigned(presigning_config)
        .await
        .map(|response| response.uri().to_string())
        .unwrap_or_else(|e| {
            log::error!("Failed to get signed URL: {:?}", e);
            "".to_string()
        })
}

pub fn extract_bucket_and_blob_from_url(url: &str) -> (String, String) {
    if url.starts_with(constants::PUBLIC_S3_URL) {
        // remove the https://storage.kjhjason.com/ prefix
        let obj_name = &url[constants::PUBLIC_S3_URL.len()..];
        let obj_name = obj_name.trim_start_matches('/');
        return (constants::BUCKET.to_string(), obj_name.to_string());
    }

    // https://github.com/rust-lang/regex?tab=readme-ov-file#usage-avoid-compiling-the-same-regex-in-a-loop
    static R2_REGEX: Lazy<regex::Regex> = Lazy::new(||
        // e.g. https://kjhjason.123456789abcdef.r2.cloudflarestorage.com/test.txt
        regex::Regex::new(r"https://(?P<bucket>[\w-]+)\.[\da-zA-Z]+\.r2\.cloudflarestorage\.com/(?P<object>.+)").unwrap());

    let captures = R2_REGEX.captures(url);
    if captures.is_none() {
        log::error!("Failed to extract bucket and object from URL: {}", url);
        return ("".to_string(), "".to_string());
    }

    let captures = captures.unwrap();
    let bucket = captures
        .name("bucket")
        .map(|m| m.as_str())
        .unwrap_or_else(|| {
            log::error!("Failed to extract bucket from URL: {}", url);
            ""
        });
    let obj_name = captures
        .name("object")
        .map(|m| m.as_str())
        .unwrap_or_else(|| {
            log::error!("Failed to extract object from URL: {}", url);
            ""
        });
    (bucket.to_string(), obj_name.to_string())
}

pub fn remove_file_from_md_content(content: &mut String, file_url: &str) -> String {
    let mut file_url = file_url.to_string();
    if let Some(index) = file_url.find('?') {
        // if ?x-id=GetObject&X-Amz-Algorithm=... is present, remove it
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

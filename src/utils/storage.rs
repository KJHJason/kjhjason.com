use aws_sdk_s3::operation::{
    copy_object::{CopyObjectError, CopyObjectOutput},
    delete_object::{DeleteObjectError, DeleteObjectOutput},
    put_object::{PutObjectError, PutObjectOutput},
};
use aws_sdk_s3::{error::SdkError, primitives::ByteStream, Client};

pub async fn upload_blob(
    client: &Client,
    bucket: &str,
    obj_name: &str,
    file_path: &str,
) -> Result<PutObjectOutput, SdkError<PutObjectError>> {
    let body = ByteStream::from_path(file_path).await;
    client
        .put_object()
        .bucket(bucket)
        .key(obj_name)
        .body(body.unwrap())
        .send()
        .await
}

pub async fn copy_blob(
    client: &Client,
    src_bucket: &str,
    src_obj_name: &str,
    dest_bucket: &str,
    dest_obj_name: &str,
) -> Result<CopyObjectOutput, SdkError<CopyObjectError>> {
    // source_bucket/source_name
    let mut source_obj_with_bucket: String = "".to_owned();
    source_obj_with_bucket.push_str(src_bucket);
    source_obj_with_bucket.push_str("/");
    source_obj_with_bucket.push_str(src_obj_name);

    return client
        .copy_object()
        .copy_source(source_obj_with_bucket)
        .bucket(dest_bucket)
        .key(dest_obj_name)
        .send()
        .await;
}

pub async fn delete_blob(
    client: &Client,
    bucket: &str,
    name: &str,
) -> Result<DeleteObjectOutput, SdkError<DeleteObjectError>> {
    return client.delete_object().bucket(bucket).key(name).send().await;
}

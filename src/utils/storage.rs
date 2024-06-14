use google_cloud_storage::client::Client;
use google_cloud_storage::http::objects::copy::CopyObjectRequest;
use google_cloud_storage::http::objects::delete::DeleteObjectRequest;
use google_cloud_storage::http::objects::upload::{Media, UploadObjectRequest, UploadType};

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

use crate::constants::constants;
use crate::database::db::DbClient;
use crate::models::blog::Blog;
use crate::models::session::Session;
use crate::models::user::User;
use crate::security::pw_hasher;
use bson::doc;
use mongodb::options::{ClientOptions, IndexOptions, ServerApi, ServerApiVersion};
use mongodb::{Client, Collection, IndexModel};

async fn init_user_collection(client: &Client) {
    let db = client.database(constants::DATABASE);
    let collection: Collection<User> = db.collection(constants::USER_COLLECTION);

    // check if the collection already exists
    let result = collection.find_one(None, None).await;
    match result {
        Ok(Some(_)) => return,
        _ => {}
    }

    // although there will only be one account, just do this for future-proofing
    let opts = IndexOptions::builder().unique(true).build();
    let index = IndexModel::builder()
        .keys(doc! {"username": 1})
        .options(opts)
        .build();
    collection
        .create_index(index, None)
        .await
        .expect("Failed to create username index for user collection");

    let admin_username =
        std::env::var(constants::BLOG_ADMIN_USERNAME).expect("admin username not set");
    let admin_password =
        std::env::var(constants::BLOG_ADMIN_PASSWORD).expect("admin password not set");
    let hashed_admin_password = tokio::task::spawn_blocking(move || {
        pw_hasher::hash_password(&admin_password).expect("Failed to hash password")
    })
    .await
    .expect("Failed to hash password");

    let user = User::new(admin_username, hashed_admin_password);
    match collection.insert_one(user, None).await {
        Ok(_) => log::info!("Admin account created"),
        Err(e) => panic!("Failed to create admin account: {}", e),
    }
    log::info!("User collection initialised");
}

async fn init_session_collection(client: &Client) {
    let db = client.database(constants::DATABASE);
    let collection: Collection<Session> = db.collection(constants::SESSION_COLLECTION);

    // check if the collection already exists
    let result = collection.find_one(None, None).await;
    match result {
        Ok(Some(_)) => return,
        _ => {}
    }

    let opts = IndexOptions::builder()
        .expire_after(std::time::Duration::from_secs(
            constants::SESSION_TIMEOUT as u64,
        ))
        .build();
    let index = IndexModel::builder()
        .keys(doc! { "created": 1 })
        .options(opts)
        .build();
    collection
        .create_index(index, None)
        .await
        .expect("Failed to create session index for session collection");

    log::info!("Session collection initialised");
}

async fn init_blog_collection(client: &Client) {
    let db = client.database(constants::DATABASE);
    let collection: Collection<Blog> = db.collection(constants::BLOG_COLLECTION);

    // check if the collection already exists
    let result = collection.find_one(None, None).await;
    match result {
        Ok(Some(_)) => return,
        _ => {}
    }

    let title_idx = IndexModel::builder().keys(doc! { "title": 1 }).build();
    let title_idx_future = collection.create_index(title_idx, None);

    let tag_idx = IndexModel::builder().keys(doc! { "tags": 1 }).build();
    let tag_idx_future = collection.create_index(tag_idx, None);

    let (title_result, tag_result) = tokio::join!(title_idx_future, tag_idx_future);
    let mut has_error = false;
    match title_result {
        Ok(_) => {}
        Err(e) => {
            has_error = true;
            log::error!("Failed to create title index: {}", e)
        }
    }
    match tag_result {
        Ok(_) => {}
        Err(e) => {
            has_error = true;
            log::error!("Failed to create tag index: {}", e)
        }
    }
    if has_error {
        panic!("Failed to create indexes for api collection");
    } else {
        log::info!("Blog collection initialised");
    }
}

pub async fn init_db() -> Result<DbClient, mongodb::error::Error> {
    let uri = if constants::get_debug_mode() {
        constants::LOCAL_URI.to_string()
    } else {
        std::env::var(constants::MONGODB_URI).expect("MONGODB_URI not set in production mode")
    };

    let mut client_options = ClientOptions::parse(uri.clone()).await?;
    client_options.app_name = Some(constants::APP_NAME.to_string());
    if !constants::get_debug_mode() {
        // Note: the code below is according to Atlas's connection instructions
        // Set the server_api field of the client_options object
        // to set the version of the Stable API on the client.
        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);
    }

    let client = match Client::with_options(client_options) {
        Ok(client) => DbClient::new(client),
        Err(e) => return Err(e),
    };
    log::info!("pinging main database to test the connection...");
    match client
        .get_database(None)
        .run_command(doc! { "ping": 1 }, None)
        .await
    {
        Ok(_) => {
            log::info!("pinged main database successfully");
        }
        Err(e) => {
            log::error!("failed to ping main database: {:?}", e);
        }
    }

    let client_ref = client.get_client();
    let init_user_future = init_user_collection(client_ref);
    let init_session_future = init_session_collection(client_ref);
    let init_blog_future = init_blog_collection(client_ref);
    tokio::join!(init_user_future, init_session_future, init_blog_future);

    Ok(client)
}

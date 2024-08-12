use crate::constants;
use crate::database::db::DbClient;
use crate::models::blog::Blog;
use crate::models::session::Session;
use crate::models::{blog, session, user, user::User};
use crate::security::pw_hasher;

use bson::doc;
use mongodb::options::{ClientOptions, IndexOptions, ServerApi, ServerApiVersion};
use mongodb::{Client, Collection, IndexModel};

async fn init_user_collection(client: &Client) {
    let db = client.database(constants::DATABASE);
    let collection: Collection<User> = db.collection(constants::USER_COLLECTION);

    // check if the collection already exists
    let admin_username =
        std::env::var(constants::BLOG_ADMIN_USERNAME).expect("admin username should be set");
    let result = collection
        .find_one(doc! {user::USERNAME_KEY: &admin_username})
        .await;
    match result {
        Ok(Some(_)) => {
            log::info!("admin account already exists");
            return;
        }
        Ok(None) => {
            log::info!("admin account does not exist");
        }
        Err(e) => {
            panic!("Failed to check if admin account exists: {}", e);
        }
    }

    let admin_email =
        std::env::var(constants::BLOG_ADMIN_EMAIL).expect("admin email should be set");

    // although there will only be one account, just do this for future-proofing
    let opts = IndexOptions::builder().unique(true).build();
    let index = IndexModel::builder()
        .keys(doc! {user::USERNAME_KEY: 1})
        .options(opts)
        .build();
    collection
        .create_index(index)
        .await
        .expect("Should be able to create username index for user collection");

    let opts = IndexOptions::builder().unique(true).build();
    let index = IndexModel::builder()
        .keys(doc! {user::EMAIL_KEY: 1})
        .options(opts)
        .build();
    collection
        .create_index(index)
        .await
        .expect("Should be able to create email index for user collection");

    let admin_password =
        std::env::var(constants::BLOG_ADMIN_PASSWORD).expect("admin password should be set");

    let hash_admin_pass_future =
        tokio::task::spawn_blocking(move || pw_hasher::hash_password(&admin_password).unwrap());
    let hashed_admin_password = hash_admin_pass_future
        .await
        .expect("Should be able to hash admin password");

    let user = User::new(admin_username, admin_email, hashed_admin_password, None);
    match collection.insert_one(user).await {
        Ok(_) => log::info!("Admin account created"),
        Err(e) => panic!("Failed to create admin account: {}", e),
    }
    log::info!("User collection initialised");
}

async fn init_session_collection(client: &Client) {
    let db = client.database(constants::DATABASE);
    let collection: Collection<Session> = db.collection(constants::SESSION_COLLECTION);

    // check if the collection already exists
    let result = collection.find_one(doc! {}).await;
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
        .keys(doc! {session::EXPIRY_KEY: 1})
        .options(opts)
        .build();
    collection
        .create_index(index)
        .await
        .expect("Should be able to create session index for session collection");

    log::info!("Session collection initialised");
}

async fn init_blog_collection(client: &Client) {
    let db = client.database(constants::DATABASE);
    let collection: Collection<Blog> = db.collection(constants::BLOG_COLLECTION);

    // check if the collection already exists
    let result = collection.find_one(doc! {}).await;
    match result {
        Ok(Some(_)) => return,
        _ => {}
    }

    let title_idx = IndexModel::builder()
        .keys(doc! {blog::TITLE_KEY: 1})
        .build();
    let title_result = collection.create_index(title_idx).await;

    let tag_idx = IndexModel::builder().keys(doc! {blog::TAGS_KEY: 1}).build();
    let tag_result = collection.create_index(tag_idx).await;

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
        std::env::var(constants::MONGODB_URI).expect("MONGODB_URI should be set in production mode")
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
        .run_command(doc! {"ping": 1})
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

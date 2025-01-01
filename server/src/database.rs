use mongodb::Database;
use tokio::sync::OnceCell;
use wither::mongodb;

use crate::settings::SETTINGS;

static CONNECTION: OnceCell<Database> = OnceCell::const_new();

pub async fn connection() -> &'static Database {
    CONNECTION
        .get_or_init(|| async {
            let db_uri = SETTINGS.database.uri.as_str();
            let db_name = SETTINGS.database.name.as_str();

            mongodb::Client::with_uri_str(db_uri)
                .await
                .expect("Failed to initialize MongoDB connection")
                .database(db_name)
        })
        .await
}

// use mongodb::{
//     bson::doc,
//     options::{ClientOptions, ServerApi, ServerApiVersion},
//     Client,
// };

// #[tokio::main]
// async fn main() -> mongodb::error::Result<()> {
//     let mut client_options =
//         ClientOptions::parse("mongodb+srv://ryotarofr:tarou.FR0608@mijinko.30pjp.mongodb.net/?retryWrites=true&w=majority&appName=mijinko")
//             .await?;

//     // Set the server_api field of the client_options object to set the version of the Stable API on the client
//     let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
//     client_options.server_api = Some(server_api);

//     // Get a handle to the cluster
//     let client = Client::with_options(client_options)?;

//     // Ping the server to see if you can connect to the cluster
//     client
//         .database("admin")
//         .run_command(doc! {"ping": 1})
//         .await?;
//     println!("Pinged your deployment. You successfully connected to MongoDB!");

//     Ok(())
// }

use std::{env, net::SocketAddr};

use dotenvy::dotenv;

use crate::{container::obtenir_controladors, routes::{get_router}};

mod controladors;
mod serveis;
mod dades;
mod routes;
mod errors;
mod container;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {

    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL no esta configurat");
    let database_type = env::var("DATABASE_TYPE").expect("DATABASE_TYPE no esta configurat");

    let controladors = obtenir_controladors(database_type, database_url).await;

    let app = get_router(controladors);
    

    let addr = SocketAddr::from(([0, 0, 0, 0], 3333));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app ).await.unwrap();

    
    Ok(())


}
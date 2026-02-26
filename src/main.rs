use std::{net::SocketAddr};

use state::{DATABASE_TYPE, DATABASE_URL};

use crate::{container::obtenir_controladors, routes::{get_router}};

mod controladors;
mod serveis;
mod dades;
mod routes;
mod errors;
mod container;
mod state;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {

    let controladors = obtenir_controladors(DATABASE_TYPE.clone(), DATABASE_URL.clone()).await;

    let app = get_router(controladors);
    

    let addr = SocketAddr::from(([0, 0, 0, 0], 3333));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app ).await.unwrap();

    
    Ok(())


}
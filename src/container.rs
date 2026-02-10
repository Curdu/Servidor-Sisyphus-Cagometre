use std::sync::Arc;

use crate::{controladors::user_controller::{UserControlador, UserController}, dades::repositoris::postgres::postgres_user_repository::PostgresUserRepository, serveis::user_service::UserServei};
use sqlx::{ postgres::PgPoolOptions};

pub(crate) async fn obtenir_controladors(type_bbdd : String, url_bbdd : String) -> impl UserController {

    if type_bbdd == "supabase" {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&url_bbdd).await.unwrap();
        println!("Connectat a la BBDD");
        let usuari_repository = PostgresUserRepository::new(pool.clone());
        let usuari_service = UserServei::new(Arc::new(usuari_repository));
        UserControlador::new(Arc::new(usuari_service))

    }else {
        panic!("Aquesta base de dades no esta suportada")
    }


}
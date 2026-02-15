use std::sync::Arc;

use crate::{controladors::{lavabo_controller::{LavaboControlador}, user_controller::{UserControlador}}, dades::repositoris::postgres::{postgres_lavabo_repository::PostgresLavaboRepository, postgres_user_repository::PostgresUserRepository}, routes::Controladors, serveis::{lavabo_service::LavaboServei, user_service::UserServei}};
use sqlx::{ postgres::PgPoolOptions};

pub(crate) async fn obtenir_controladors(type_bbdd : String, url_bbdd : String) -> Controladors {

    if type_bbdd == "supabase" {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&url_bbdd).await.unwrap();
        println!("Connectat a la BBDD");

        let usuari_repository = PostgresUserRepository::new(pool.clone());
        let usuari_service = UserServei::new(Arc::new(usuari_repository));

        let lavabo_repository = PostgresLavaboRepository::new(pool.clone());
        let lavabo_service = LavaboServei::new(Arc::new(lavabo_repository));

        let user_controller = UserControlador::new(Arc::new(usuari_service));
        let lavabo_controller = LavaboControlador::new(Arc::new(lavabo_service));
        Controladors{usuari : Arc::new(user_controller), lavabo : Arc::new(lavabo_controller)}

    }else {
        panic!("Aquesta base de dades no esta suportada")
    }


}
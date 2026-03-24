use std::sync::Arc;

use crate::{controladors::{auth_controller::AuthControlador, etiqueta_controller::{EtiquetaControlador}, lavabo_controller::LavaboControlador, resenya_controller::ResenyaControlador, user_controller::UserControlador}, dades::repositoris::postgres::{postgres_etiqueta_repository::PostgresEtiquetaRepository, postgres_lavabo_repository::PostgresLavaboRepository, postgres_resenya_repository::PostgresResenyaRepository, postgres_user_repository::PostgresUserRepository}, routes::Controladors, serveis::{auth_service::AuthServei, etiqueta_service::EtiquetaServei, lavabo_service::LavaboServei, resenya_service::ResenyaServei, user_service::UserServei}};
use sqlx::{ postgres::PgPoolOptions};

pub(crate) async fn obtenir_controladors(type_bbdd : String, url_bbdd : String) -> Controladors {

    if type_bbdd == "supabase" {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&url_bbdd).await.unwrap();
        println!("Connectat a la BBDD");

        let usuari_repository = Arc::new(PostgresUserRepository::new(pool.clone()));
        let usuari_service = UserServei::new(usuari_repository.clone());

        let lavabo_repository = PostgresLavaboRepository::new(pool.clone());
        let lavabo_service = LavaboServei::new(Arc::new(lavabo_repository));

        let auth_service = AuthServei::new(usuari_repository.clone());

        let resenya_repository = Arc::new(PostgresResenyaRepository::new(pool.clone()));
        let resenya_service = Arc::new(ResenyaServei::new(resenya_repository));

        let etiqueta_repository = Arc::new(PostgresEtiquetaRepository::new(pool.clone()));
        let etiqueta_service = Arc::new(EtiquetaServei::new(etiqueta_repository));


        let user_controller = Arc::new(UserControlador::new(Arc::new(usuari_service)));
        let lavabo_controller = Arc::new(LavaboControlador::new(Arc::new(lavabo_service)));
        let auth_controller = Arc::new(AuthControlador::new(Arc::new(auth_service)));
        let resenya_controller = Arc::new(ResenyaControlador::new(resenya_service));
        let etiqueta_controller = Arc::new(EtiquetaControlador::new(etiqueta_service));
        
        Controladors{usuari : user_controller, lavabo : lavabo_controller, auth: auth_controller, resenya: resenya_controller, etiqueta: etiqueta_controller}

    }else {
        panic!("Aquesta base de dades no esta suportada")
    }


}
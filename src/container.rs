use std::sync::Arc;

use crate::{controladors::{auth_controller::AuthControlador, etiqueta_controller::EtiquetaControlador, lavabo_controller::LavaboControlador, resenya_controller::ResenyaControlador, user_controller::UserControlador}, dades::repositoris::{postgres::{postgres_etiqueta_repository::PostgresEtiquetaRepository, postgres_lavabo_imatge_repository::PostgresLavaboImatgeRepository, postgres_lavabo_repository::PostgresLavaboRepository, postgres_resenya_repository::PostgresResenyaRepository, postgres_user_repository::PostgresUserRepository}, supabase_storage::supabase_lavabo_images_repository::SupabaseImatgesRepository}, routes::Controladors, serveis::{auth_service::AuthServei, etiqueta_service::EtiquetaServei, lavabo_service::LavaboServei, resenya_service::ResenyaServei, user_service::UserServei}};
use sqlx::{ postgres::PgPoolOptions};
use tokio::sync::Mutex;

pub(crate) async fn obtenir_controladors(type_bbdd : String, url_bbdd : String) -> Controladors {

    if type_bbdd == "supabase" {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&url_bbdd).await.unwrap();
        println!("Connectat a la BBDD");
        let client = reqwest::Client::new();


        //Repositoris
        let usuari_repository = Arc::new(PostgresUserRepository::new(pool.clone()));
        let lavabo_repository = PostgresLavaboRepository::new(pool.clone());
        let image_repository = SupabaseImatgesRepository{client: Arc::new(Mutex::new(client))};
        let lavabo_imatge_repository = PostgresLavaboImatgeRepository::new(pool.clone());
        let auth_service = AuthServei::new(usuari_repository.clone());
        let resenya_repository = Arc::new(PostgresResenyaRepository::new(pool.clone()));
        let etiqueta_repository = Arc::new(PostgresEtiquetaRepository::new(pool.clone()));

        //Serveis
        let etiqueta_service = Arc::new(EtiquetaServei::new(etiqueta_repository));
        let resenya_service = Arc::new(ResenyaServei::new(resenya_repository));
        let lavabo_service = LavaboServei::new(Arc::new(lavabo_repository), Arc::new(image_repository), Arc::new(lavabo_imatge_repository));
        let usuari_service = UserServei::new(usuari_repository.clone());



        //Controladors
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
use async_trait::async_trait;
use sqlx::{PgPool, query, query_as};
use uuid::Uuid;

use crate::{dades::{models::lavabo::LavaboImatge, repositoris::traits::lavabo_imatge_repository::LavaboImatgeRepository}, errors::crud_errors::CrudErrors};

pub(crate) struct PostgresLavaboImatgeRepository {
    bd: PgPool
}

impl PostgresLavaboImatgeRepository {
    pub fn new (bd : PgPool) -> Self {
        Self { bd }
    }
}

#[async_trait]
impl LavaboImatgeRepository for PostgresLavaboImatgeRepository {
    async fn obtenir_imatges_per_lavabo_id(&self, lavabo_id: Uuid) -> Result<Vec<LavaboImatge>, CrudErrors> {
        let sql_query = "SELECT * FROM imatge_lavabo WHERE lavabo_id = $1";
        let lavabos = query_as::<_,LavaboImatge>(sql_query)
            .bind(lavabo_id)
            .fetch_all(&self.bd)
            .await?;

        Ok(lavabos)
    }
    async fn crear_imate_lavabo(&self, lavabo_imatge : LavaboImatge) -> Result<(), CrudErrors> {
        let sql_query = "INSERT INTO imatge_lavabo (lavabo_id, path, created_at) VALUES ($1,$2,$3)";

        let result = query(sql_query)
            .bind(lavabo_imatge.lavabo_id)
            .bind(lavabo_imatge.path)
            .bind(lavabo_imatge.created_at)
            .execute(&self.bd)
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(error) => {
                Err(error.as_database_error().unwrap().into())
            }
        }
    }
    async fn eliminar_imatge_lavabo(&self, lavabo_id: Uuid, path : String) -> Result<(), CrudErrors> {
        let sql_query = "DELETE imatge_lavabo WHERE lavabo_id = $1 AND path = $2";

        let result = query(sql_query)
            .bind(lavabo_id)
            .bind(path)
            .execute(&self.bd)
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(error) => {
                Err(error.as_database_error().unwrap().into())
            }
        }
    }
}
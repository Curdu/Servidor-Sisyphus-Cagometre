use async_trait::async_trait;
use sqlx::{PgPool, query, query_as};
use uuid::Uuid;

use crate::{dades::{models::etiqueta::Etiqueta, repositoris::traits::{compartit::irepository::IRepository, etiqueta_repository::EtiquetaRepository}}, errors::crud_errors::CrudErrors};

pub struct PostgresEtiquetaRepository {
    pub(crate) bd : PgPool
}

impl PostgresEtiquetaRepository {
    pub(crate) fn new(pg_pool: PgPool) -> Self {
        Self { bd: pg_pool }
    }
}

#[async_trait]
impl IRepository<Etiqueta> for PostgresEtiquetaRepository {
    async fn obte_per_id(&self,id: Uuid) -> Result<Etiqueta, CrudErrors> {
        let sql_query = "SELECT * FROM etiqueta WHERE id = $1";

        let result = sqlx::query_as::<_,Etiqueta>(sql_query)
            .bind(id)
            .fetch_optional(&self.bd)
            .await;

        match result? {
            Some(etiqueta) => Ok(etiqueta),
            None => Err(CrudErrors::NotFound("Etiqueta no trobada".to_string()))
        }

    }
    async fn crear(&self,model : Etiqueta) -> Result<(), CrudErrors> {
        let sql_query = "INSERT INTO etiqueta (id, created_at, nom) VALUES ($1,$2,$3)";

        let result = query(sql_query)
            .bind(model.id)
            .bind(model.created_at)
            .bind(model.nom)
            .execute(&self.bd)
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(error) => {
                Err(error.as_database_error().unwrap().into())
            }
        }
    }
    async fn actualitzar(&self,id: Uuid, model : Etiqueta) -> Result<(), CrudErrors> {
        let sql_query = "UPDATE etiqueta SET nom = $1 WHERE id = $2";

        let result = query(sql_query)
            .bind(model.nom)
            .bind(id)
            .execute(&self.bd)
            .await;

        if result?.rows_affected() == 0 {
            Err(CrudErrors::NotFound("Etiqueta no trobada".to_string()))
        } else {
            Ok(())
        }
    }
    async fn eliminar(&self,id: Uuid) -> Result<(), CrudErrors> {
        let sql_query = "DELETE from etiqueta WHERE id= $1";

        let result = query(sql_query)
            .bind(id)
            .execute(&self.bd)
            .await;

        if result?.rows_affected() == 0 {
            Err(CrudErrors::NotFound("Etiqueta no trobada".to_string()))
        } else {
            Ok(())
        }
    }
}
#[async_trait]
impl EtiquetaRepository for PostgresEtiquetaRepository {
    async fn obte_totes_etiquetes(&self) -> Result<Vec<Etiqueta>, CrudErrors>{
        let sql_query = r#"SELECT * FROM etiqueta"#;

        let lavabos = query_as::<_,Etiqueta>(sql_query)
            .fetch_all(&self.bd)
            .await;
        Ok(lavabos?)
    }
}

use async_trait::async_trait;
use sqlx::{PgPool, query};
use uuid::Uuid;

use crate::{dades::{models::resenya::Resenya, repositoris::traits::compartit::irepository::IRepository}, errors::crud_errors::CrudErrors};

pub(crate) struct PostgresResenyaRepository {
    bd : PgPool
}

impl PostgresResenyaRepository {
    pub fn new (bd: PgPool)-> Self {
        Self { bd }
    }
}

#[async_trait]
impl IRepository<Resenya> for PostgresResenyaRepository {
    async fn obte_per_id(&self,id: Uuid) -> Result<Resenya, CrudErrors> {
        let resultat = sqlx::query_as::<_,Resenya>("SELECT * FROM resenya WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.bd)
            .await;

            match resultat?  { 
                Some(resenya) => Ok(resenya),
                None => Err(CrudErrors::NotFound("Resenya no trobada".to_string()))
            } 
    }
    
    async fn crear(&self,model : Resenya) -> Result<(), CrudErrors> {
        let sql_query = "INSERT INTO resenya (id, created_at, id_lavabo,id_usuari,comentari,puntuacio) VALUES($1,$2,$3,$4,$5,$6)";

        let resultat = query(sql_query)
            .bind(model.id)
            .bind(model.created_at)
            .bind(model.id_lavabo)
            .bind(model.id_usuari)
            .bind(model.comentari)
            .bind(model.puntuacio)
            .execute(&self.bd)
            .await;
        
        match resultat {
            Ok(_) => Ok(()),
            Err(error) => {
                let database_error = error.as_database_error().unwrap();
                Err(database_error.into())
            }
        }
        
    }
    async fn actualitzar(&self,id: Uuid, model : Resenya) -> Result<(), CrudErrors> {
        let sql_query = "UPDATE resenya SET comentari = $1, puntuacio = $2 WHERE id = $3 AND id_usuari = $4";

        let result = query(sql_query)
            .bind(model.comentari)
            .bind(model.puntuacio)
            .bind(id)
            .bind(model.id_usuari)
            .execute(&self.bd)
            .await;

        if result?.rows_affected() == 0 {
            Err(CrudErrors::NotFound("Resenya no trobada".to_string()))
        } else {
            Ok(())
        }

    }
    async fn eliminar(&self,id: Uuid) -> Result<(), CrudErrors> {
        let sql_query = "DELETE from resenya WHERE id = $1";

        let resultat = query(sql_query)
            .bind(id)
            .execute(&self.bd)
            .await;

        if resultat?.rows_affected() == 0 {
            Err(CrudErrors::NotFound("Resenya no trobada".to_string()))
        } else {
            Ok(())
        }

    }
}
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use sqlx::{query,query_as, Error};

use crate::dades::models::lavabo::{Lavabo, LavaboAmbEtiquetes};
use crate::errors::lavabo_errors::LavaboErrors;
use crate::dades::repositoris::traits::lavabo_repository::LavaboRepository;

pub(crate) struct PostgresLavaboRepository {
    bd : PgPool
}

impl PostgresLavaboRepository {
    pub(crate) fn new(bd: PgPool) -> Self {
        Self { bd }
    }
}

#[async_trait]
impl LavaboRepository for PostgresLavaboRepository {
    async fn obte_lavabo_per_id(&self,id: Uuid) -> Result<Lavabo, LavaboErrors>{
        let result = query_as::<_, Lavabo>("SELECT * FROM lavabo WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.bd).await;
        match result {
            Ok(option) =>{
                match option {
                    Some(lavabo) => Ok(lavabo),
                    None => Err(LavaboErrors::LavaboNotFound("El lavabo no s'ha trobat".to_string()))
                }
            },
            Err(error) => Err(LavaboErrors::ServerError(error.to_string()))
        }
    }
    async fn crear_lavabo(&self,lavabo : Lavabo) -> Result<(), LavaboErrors>{
        let sql_query = r#"INSERT INTO lavabo (id,created_at,puntuacio_mitja,descripcio, nombre_resenyes, titol) VALUES ($1,$2,$3,$4,$5,$6)"#;

        let result = query(sql_query)
            .bind(lavabo.id.clone())
            .bind(lavabo.created_at)
            .bind(lavabo.puntuacio_mitja)
            .bind(lavabo.descripcio)
            .bind(lavabo.nombre_resenyes)
            .bind(lavabo.titol)
            .execute(&self.bd).await;

                match result {
            Ok(_) =>{
                Ok(())
            },
            Err(error) => {
               match error {
                Error::Database(db_error) => {
                    let codi = db_error.code().unwrap();
                    if codi == "23505" {
                        if db_error.constraint().unwrap() == "lavabo_id_key" {
                            Err(LavaboErrors::LavaboExistent(format!("El lavabo amb el id {} ja existeix", lavabo.id)))
                        } else {
                            Err(LavaboErrors::ServerError(db_error.to_string()))
                        }
                    }else {
                        Err(LavaboErrors::ServerError(db_error.to_string()))
                    }
                },
                _ => {
                    Err(LavaboErrors::ServerError("Error desconegut".to_string()))
                }
               }
            }
            
        }
        

    }
    async fn actualitzar_lavabo(&self,id: Uuid, lavabo : Lavabo) -> Result<(), LavaboErrors>{
        let sql_query = r#"UPDATE lavabo SET descripcio = $1, titol = $2 WHERE id = $3"#;
        let result = query(sql_query)
            .bind(lavabo.descripcio)
            .bind(lavabo.titol)
            .bind(id)
            .execute(&self.bd).await;

        match result {
            Ok(pg_result)=>{
                if pg_result.rows_affected() == 0 {
                    Err(LavaboErrors::LavaboNotFound("El lavabo no s'ha trobat".to_string()))
                } else {
                    Ok(())
                }
            },
            Err(error) => Err(LavaboErrors::ServerError(error.to_string()))
            
        }
    }
    async fn eliminar_lavabo(&self,id: Uuid) -> Result<(), LavaboErrors>{
        let sql_query = r#"DELETE from lavabo WHERE id = $1"#;

        let result = query(sql_query)
            .bind(id)
            .execute(&self.bd).await;

        match result {
            Ok(pg_result) => {
                if pg_result.rows_affected() == 0 {
                    Err(LavaboErrors::LavaboNotFound("El lavabo no s'ha trobat".to_string()))
                }else {
                    Ok(())
                }
            },
            Err(error) => Err(LavaboErrors::ServerError(error.to_string()))
            
        }
    }

    async fn obte_tots_lavabos(&self) -> Result<Vec<Lavabo>, LavaboErrors> {
        let sql_query = r#"SELECT * FROM lavabo"#;
        let result = query_as::<_,Lavabo>(sql_query)
            .fetch_all(&self.bd).await;

        match result {
            Ok(lavabos) => Ok(lavabos),
            Err(error) => Err(LavaboErrors::ServerError(error.to_string()))
        }
        
    }
    async fn obte_tots_lavabos_amb_etiquetes(&self) -> Result<Vec<LavaboAmbEtiquetes>, LavaboErrors> {
        let sql_query = r#"
        SELECT 
            l.id, 
            l.created_at, 
            l.descripcio, 
            l.titol, 
            l.puntuacio_mitja, 
            l.nombre_resenyes,
            COALESCE(
                JSON_AGG(
                    JSON_BUILD_OBJECT('id', e.id, 'nom', e.nom, 'created_at', e.created_at)
                ) FILTER (WHERE e.id IS NOT NULL),
                 '[]'
            ) AS etiquetes
        FROM lavabo l 
        LEFT JOIN lavabo_etiqueta le ON l.id = le.id_lavabo
        LEFT JOIN etiqueta e ON le.id_etiqueta = e.id
        GROUP BY l.id"#;

        let result = query_as::<_,LavaboAmbEtiquetes>(sql_query)
            .fetch_all(&self.bd)
            .await;
        match result {
            Ok(lavabos) => Ok(lavabos),
            Err(error) => Err(LavaboErrors::ServerError(error.to_string()))
            
        }
    }
}
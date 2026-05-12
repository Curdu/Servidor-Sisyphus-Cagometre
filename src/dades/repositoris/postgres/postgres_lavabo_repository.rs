use async_trait::async_trait;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
use sqlx::{query,query_as, Error};

use crate::dades::models::lavabo::{Lavabo, LavaboAmbEtiquetes, LavaboDetallat};
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
        let sql_query = r#"INSERT INTO lavabo (id,created_at,puntuacio_mitja,descripcio, nombre_resenyes, titol, creador_id, localitzacio) VALUES ($1,$2,$3,$4,$5,$6,$7,$8)"#;

        let result = query(sql_query)
            .bind(lavabo.id.clone())
            .bind(lavabo.created_at)
            .bind(lavabo.puntuacio_mitja)
            .bind(lavabo.descripcio)
            .bind(lavabo.nombre_resenyes)
            .bind(lavabo.titol)
            .bind(lavabo.creador_id)
            .bind(lavabo.localitzacio)
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

    async fn afegir_etiquetes_lavabo(&self, etiqueta : Uuid, id_lavabo: Uuid) -> Result<(), LavaboErrors> {
        let sql_query = r#"INSERT INTO lavabo_etiqueta (id_lavabo,id_etiqueta,created_at) VALUES($1,$2,$3)"#;
        let result = sqlx::query(sql_query)
            .bind(id_lavabo)
            .bind(etiqueta)
            .bind(Utc::now())
            .execute(&self.bd)
            .await;
        if let Err(error) = result {
            let db_err = error.as_database_error().unwrap();
            if db_err.code().unwrap() == "23503" {
                return Err(LavaboErrors::ServerError("El id del lavabo o de la etiqueta no existeix".to_string()))
            } else {
                return Err(LavaboErrors::ServerError(db_err.message().to_string()))
            } 
        }
        
        Ok(())
    }

    async fn eliminar_etiqueta_lavabo(&self, etiqueta : Uuid, id_lavabo: Uuid) -> Result<(), LavaboErrors> {
        let sql_query = r#"DELETE FROM lavabo_etiqueta WHERE id_lavabo = $1 AND id_etiqueta = $2"#;
        let result = sqlx::query(sql_query)
            .bind(id_lavabo)
            .bind(etiqueta)
            .execute(&self.bd)
            .await;
        match result {
            
            Ok(query_result) =>{
                if query_result.rows_affected() == 0 {
                    Err(LavaboErrors::LavaboNotFound("No s'ha trobat el lavabo i la etiqueta amb els ids especificats".to_string()))
                }else {
                    Ok(())
                }
            },
            Err(error) => Err(LavaboErrors::ServerError(error.to_string()))
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
            l.creador_id,
            l.localitzacio,
            (
                SELECT COALESCE(
                    JSON_AGG(
                        JSON_BUILD_OBJECT('id', e.id, 'nom', e.nom, 'created_at', e.created_at)
                    ), '[]'
                )
                FROM lavabo_etiqueta le
                JOIN etiqueta e ON le.id_etiqueta = e.id
                WHERE le.id_lavabo = l.id
            ) AS etiquetes,

            
            (
                SELECT COALESCE(
                    JSON_AGG(
                        JSON_BUILD_OBJECT('lavabo_id', il.lavabo_id, 'path', il.path, 'created_at', il.created_at)
                    ), '[]'
                )
                FROM imatge_lavabo il
                WHERE il.lavabo_id = l.id
            ) AS imatges
        FROM lavabo l "#;

        let result = query_as::<_,LavaboAmbEtiquetes>(sql_query)
            .fetch_all(&self.bd)
            .await;
        match result {
            Ok(lavabos) => Ok(lavabos),
            Err(error) => Err(LavaboErrors::ServerError(error.to_string()))
            
        }
    }
    async fn obte_lavabo_detallat_per_id(&self, id: Uuid) -> Result<LavaboDetallat, LavaboErrors> {
        let sql_query = r#"
        SELECT 
    l.id, 
    l.created_at, 
    l.descripcio, 
    l.titol, 
    l.puntuacio_mitja, 
    l.nombre_resenyes,
    l.creador_id,
    l.localitzacio,
    
    (
        SELECT COALESCE(
            JSON_AGG(
                JSON_BUILD_OBJECT('id', e.id, 'nom', e.nom, 'created_at', e.created_at)
            ), '[]'
        )
        FROM lavabo_etiqueta le
        JOIN etiqueta e ON le.id_etiqueta = e.id
        WHERE le.id_lavabo = l.id
    ) AS etiquetes,

    
    (
        SELECT COALESCE(
            JSON_AGG(
                JSON_BUILD_OBJECT('lavabo_id', il.lavabo_id, 'path', il.path, 'created_at', il.created_at)
            ), '[]'
        )
        FROM imatge_lavabo il
        WHERE il.lavabo_id = l.id
    ) AS imatges,

    
    (
        SELECT COALESCE(
            JSON_AGG(
                JSON_BUILD_OBJECT('id', r.id, 'id_lavabo', r.id_lavabo, 'created_at', r.created_at, 'id_usuari', r.id_usuari, 'comentari', r.comentari, 'puntuacio', r.puntuacio, 'nom_usuari', usuari.nom, 'imatge_perfil', usuari.imatge_url)
            ), '[]'
        )
        FROM resenya r
        JOIN usuari ON usuari.id = r.id_usuari
        WHERE r.id_lavabo = l.id
    ) AS resenyes

    FROM lavabo l 
    WHERE l.id = $1"#;

        let result = query_as::<_,LavaboDetallat>(sql_query)
            .bind(id)
            .fetch_optional(&self.bd)
            .await;

        match result {
            Ok(opt) =>{
                match opt {
                    Some(lavabo) => Ok(lavabo),
                    None => Err(LavaboErrors::LavaboNotFound(format!("El lavabo amb l'id {} no existeix", id.to_string())))
                }
            },
            Err(err) => Err(LavaboErrors::ServerError(err.to_string()))
            
        }
    }
    async fn modificar_puntuacio(&self, puntuacio_mitja: f32, nombre_resenyes: i64, id_lavabo: Uuid) -> Result<(), LavaboErrors> {
        let sql_query = "UPDATE lavabo SET puntuacio_mitja = $1, nombre_resenyes = $2 WHERE id = $3";

        let result = sqlx::query(sql_query)
            .bind(puntuacio_mitja)
            .bind(nombre_resenyes)
            .bind(id_lavabo)
            .execute(&self.bd)
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(error) => Err(LavaboErrors::ServerError(error.to_string()))
        }

    }

}
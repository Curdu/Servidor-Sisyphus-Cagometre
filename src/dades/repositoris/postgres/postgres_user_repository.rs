
use async_trait::async_trait;
use sqlx::{Error, PgPool};
use uuid::Uuid;
use sqlx::{query_as, query};

use crate::dades::repositoris::traits::user_repository::UserRepository;
use crate::dades::models::usuari::Usuari;
use crate::errors::usuari_errors::UsuariErrors;


pub(crate) struct PostgresUserRepository {
    bd: PgPool
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn obte_usuari_per_id(&self ,id: Uuid) -> Result<Usuari,UsuariErrors> {
        let result = query_as::<_,Usuari>(r#"SELECT * FROM usuari WHERE id = $1"#)
        .bind(id)
        .fetch_optional(&self.bd).await;
    
        match result {
            Ok(usuari) => {
                match usuari {
                    Some(usr) => Ok(usr),
                    None => Err(UsuariErrors::UsuariNotFound("L'usuari no existeix".to_string()))
                }
            },
            Err(error) => {
                Err(UsuariErrors::ServerError(error.to_string()))
            }
            
        }
    }
    async fn obte_usuari_per_correu(&self ,correu: String) -> Result<Usuari, UsuariErrors>{
        let result = query_as::<_,Usuari>(r#"SELECT * FROM usuari WHERE correu = $1"#)
        .bind(correu)
        .fetch_optional(&self.bd).await;
        match result {
            Ok(option) =>{
                match option {
                    Some(user) => Ok(user),
                    None => Err(UsuariErrors::UsuariNotFound("L'usuari no s'ha trobat".to_string()))
                }
            },
            Err(error) => {
                Err(UsuariErrors::ServerError(error.to_string()))
            }
            
        }
        
    }
    async fn crear_usuari(&self ,usuari: Usuari) -> Result<(),UsuariErrors> {
        let result = query("INSERT INTO usuari (correu, created_at, nom, cognoms, hash, salt) VALUES ($1,$2,$3,$4,$5,$6)")
        .bind(usuari.correu.clone())
        .bind(usuari.created_at)
        .bind(usuari.nom)
        .bind(usuari.cognoms)
        .bind(usuari.hash)
        .bind(usuari.salt)
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
                        if db_error.constraint().unwrap() == "usuari_correu_key" {
                            Err(UsuariErrors::UsuariExistent(format!("L'usuari amb el correu {} ja existeix", usuari.correu)))
                        } else {
                            Err(UsuariErrors::ServerError(codi.to_string()))
                        }
                    }else {
                        Err(UsuariErrors::ServerError(codi.to_string()))
                    }

                },
                _ => {
                    Err(UsuariErrors::ServerError("Error desconegut".to_string()))
                }
                   
               }
            }
            
        }
    }   
    async fn actualitza_usuari(&self ,id: Uuid, nou_usuari: Usuari) -> Result<(),UsuariErrors> {
        let sql_query: &str = "UPDATE usuari SET correu = $1, nom = $2, cognoms = $3 WHERE id = $4";

        let result = query(sql_query)
            .bind(nou_usuari.correu)
            .bind(nou_usuari.nom)
            .bind(nou_usuari.cognoms)
            .bind(id)
            .execute(&self.bd).await;
        match result {
            Ok(response) => {
                if response.rows_affected() == 0 {
                   Err(UsuariErrors::UsuariNotFound("L'usuari que es volia modificar no s'ha trobat".to_string())) 
                }else {
                    Ok(())
                }
            },
            Err(error) =>{
                Err(UsuariErrors::ServerError(error.to_string()))
            }
            
        }
    }
    async fn elimina_usuari(&self ,id: Uuid) -> Result<(),UsuariErrors> {
        let sql_query = r#"DELETE FROM usuari WHERE id = $1 "#;
        let result = query(sql_query)
            .bind(id)
            .execute(&self.bd).await;

        match result {
            Ok(response) => {
                if response.rows_affected() == 0 {
                    Err(UsuariErrors::UsuariNotFound("L'usuari que es volia eliminar no s'ha trobat".to_string()))
                }else{
                    Ok(())
                }
            },
            Err(error) => {
                Err(UsuariErrors::ServerError(error.to_string()))
            }
            
        }
    }
}

impl PostgresUserRepository {
    pub fn new(bd: PgPool) -> Self {
        Self { bd }
    }
}
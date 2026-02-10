use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::dades::models::usuari::Usuari;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct UsuariDTO {
    pub(crate) id: Uuid,
    pub(crate) correu: String,
    pub(crate) nom: String,
    pub(crate) cognoms: String,
    pub(crate) contrasenya: String,
    pub(crate) created_at: DateTime<Utc>
}

impl UsuariDTO {
    pub fn new(id: Uuid, correu: String, nom: String, cognoms: String, contrasenya: String, created_at: DateTime<Utc>) -> Self {
        Self { id, correu, nom, cognoms, contrasenya, created_at }
    }
}

impl From<Usuari> for UsuariDTO {
    fn from(value: Usuari) -> Self {
        Self { 
            id: value.id, 
            correu: value.correu, 
            nom: value.nom, 
            cognoms: value.cognoms, 
            contrasenya: "".to_string(), 
            created_at: value.created_at
        }
    }
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::uuid};
use uuid::{ Uuid};

use crate::serveis::dtos::usuari_dto::UsuariDTO;

use super::rols::UsuariRol;

#[derive(Serialize,Deserialize,Debug, FromRow)]
pub(crate) struct Usuari {
    pub(crate) id: Uuid,
    pub(crate) correu: String,
    pub(crate) nom: String,
    pub(crate) cognoms: String,
    pub(crate) hash: String,
    pub(crate) salt: String,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) rol: UsuariRol
}

impl Usuari {
    pub fn new(id: Uuid, correu: String, nom: String, cognoms: String, hash: String, salt: String, created_at: DateTime<Utc>, rol: UsuariRol) -> Self {
        Self { id, correu, nom, cognoms, hash, salt, created_at, rol }
    }
}

impl From<UsuariDTO> for Usuari {
    fn from(value: UsuariDTO) -> Self {
        Self { id: value.id, correu: value.correu, nom: value.nom, cognoms: value.cognoms, hash: "".to_string(), salt: "".to_string(), created_at: value.created_at , rol: value.rol}
    }
}

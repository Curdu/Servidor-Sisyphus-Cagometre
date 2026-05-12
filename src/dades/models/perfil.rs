use serde::Serialize;
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::serveis::dtos::perfil_dto::PerfilDTO;

#[derive(Debug, Serialize, FromRow)]
pub(crate) struct Perfil {
    pub(crate) id: Uuid,
    pub(crate) nom: String,
    pub(crate) cognoms: String,
    pub(crate) imatge_url: Option<String>,
    pub(crate) nombre_de_resenyes: i64
}

impl From<PerfilDTO> for Perfil {
    fn from(value: PerfilDTO) -> Self {
        Self { id: value.id, nom: value.nom, cognoms: value.cognoms, imatge_url: value.imatge_url , nombre_de_resenyes: value.nombre_de_resenyes}
    }
}
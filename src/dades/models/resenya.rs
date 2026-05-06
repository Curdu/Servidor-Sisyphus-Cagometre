use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::{Uuid};

use crate::serveis::dtos::resenya_dto::{ResenyaAmbPerfilDTO, ResenyaDTO};

#[derive(FromRow, Debug, Deserialize)]
pub(crate) struct Resenya {
    pub(crate) id: Uuid,
    pub(crate) id_lavabo: Uuid,
    pub(crate) id_usuari: Uuid,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) comentari: String,
    pub(crate) puntuacio: f32
}

impl From<ResenyaDTO> for Resenya {
    fn from(value: ResenyaDTO) -> Self {
        Resenya { id: value.id, id_lavabo: value.id_lavabo, id_usuari: value.id_usuari, created_at: value.created_at, comentari: value.comentari, puntuacio: value.puntuacio }
    }
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub (crate) struct ResenyaAmbPerfil {
    pub(crate) id: Uuid,
    pub(crate) id_lavabo: Uuid,
    pub(crate) id_usuari: Uuid,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) comentari: String,
    pub(crate) puntuacio: f32,
    pub(crate) nom_usuari: String,
    pub(crate) imatge_perfil: Option<String>
}

impl From<ResenyaAmbPerfilDTO> for ResenyaAmbPerfil {
    fn from(value: ResenyaAmbPerfilDTO) -> Self {
        Self { id: value.id, 
            id_lavabo: value.id_lavabo, 
            id_usuari: value.id_usuari, 
            created_at: value.created_at, 
            comentari: value.comentari, 
            puntuacio: value.puntuacio, 
            nom_usuari: value.nom_usuari, 
            imatge_perfil: value.imatge_perfil }
    }
}
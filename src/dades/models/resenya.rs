use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::{Uuid};

use crate::serveis::dtos::resenya_dto::ResenyaDTO;

#[derive(FromRow)]
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
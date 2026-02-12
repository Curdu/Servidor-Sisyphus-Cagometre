use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::serveis::dtos::lavabo_dto::LavaboDTO;

#[derive(Debug,Serialize, Deserialize, FromRow)]
pub(crate) struct Lavabo {
    pub(crate) id: Uuid,
    pub(crate) descripcio: String,
    pub(crate) puntuacio: f32,
    pub(crate) created_at: DateTime<Utc> 
}

impl From<LavaboDTO> for Lavabo {
    fn from(value: LavaboDTO) -> Self {
        Self { id: value.id, descripcio: value.descripcio, puntuacio: value.puntuacio, created_at: value.created_at }
    }
}
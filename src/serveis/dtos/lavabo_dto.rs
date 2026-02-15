use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::dades::models::lavabo::Lavabo;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct LavaboDTO {
    pub(crate) id: Uuid,
    pub(crate) puntuacio: f64,
    pub(crate) descripcio: String,
    pub(crate) created_at: DateTime<Utc>
}

impl LavaboDTO {
    pub(crate) fn new(id : Uuid, puntuacio : f64, descripcio: String, created_at: DateTime<Utc>) -> Self{
        Self { id, puntuacio, descripcio, created_at }
    }
}

impl From<Lavabo> for LavaboDTO {
    fn from(value: Lavabo) -> Self {
        Self { id: value.id, puntuacio: value.puntuacio, descripcio: value.descripcio, created_at: value.created_at}
    }
}
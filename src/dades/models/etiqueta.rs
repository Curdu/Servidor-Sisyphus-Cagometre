use chrono::{DateTime, Utc};
use serde::Deserialize;
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::serveis::dtos::etiqueta_dto::EtiquetaDTO;

#[derive(FromRow, Debug, Deserialize)]
pub(crate) struct Etiqueta{
    pub(crate) id: Uuid,
    pub(crate) nom: String,
    pub(crate) created_at: DateTime<Utc>

}

impl From<EtiquetaDTO> for Etiqueta {
    fn from(value: EtiquetaDTO) -> Self {
        Etiqueta { id: value.id, nom: value.nom , created_at: value.created_at}
    }
}
use axum_typed_multipart::{FieldData, TryFromMultipart};
use chrono::Utc;
use tempfile::NamedTempFile;
use uuid::Uuid;

use crate::{ serveis::dtos::lavabo_dto::LavaboDTO};


#[derive(Debug, TryFromMultipart)]
pub(crate) struct CreateLavaboRequest {
    pub(crate) titol: String,
    pub(crate) descripcio: String,
    #[form_data(limit= "unlimited")]
    pub(crate) imatges : Vec<FieldData<NamedTempFile>>
}

impl From<CreateLavaboRequest> for (LavaboDTO, Vec<FieldData<NamedTempFile>>) {
    fn from(value: CreateLavaboRequest) -> Self {
        let lavabo_dto = LavaboDTO{id: Uuid::new_v4(), descripcio: value.descripcio, titol: value.titol, puntuacio_mitja: 0.0, nombre_resenyes: 0, created_at: Utc::now(), creador_id: Uuid::nil()};
        (lavabo_dto,value.imatges)
    }
}
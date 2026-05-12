use axum_typed_multipart::{FieldData, TryFromMultipart};
use chrono::Utc;
use tempfile::NamedTempFile;
use uuid::Uuid;

use crate::serveis::dtos::{lavabo_dto::{LavaboDTO}};


#[derive(Debug, TryFromMultipart)]
pub(crate) struct CreateLavaboRequest {
    pub(crate) titol: String,
    pub(crate) descripcio: String,
    #[form_data(limit= "unlimited")]
    pub(crate) imatges : Vec<FieldData<NamedTempFile>>,
    pub(crate) localitzacio: String,
    pub(crate) etiquetes: Vec<Uuid>,
    pub(crate) puntuacio_inicial: f32
}

impl From<CreateLavaboRequest> for (LavaboDTO, Vec<FieldData<NamedTempFile>>, Vec<Uuid>) {
    fn from(value: CreateLavaboRequest) -> Self {
        let lavabo_dto = LavaboDTO{
            id: Uuid::new_v4(), 
            descripcio: value.descripcio, 
            titol: value.titol, 
            puntuacio_mitja: value.puntuacio_inicial, 
            nombre_resenyes: 0, 
            created_at: Utc::now(), 
            creador_id: Uuid::nil(),
            localitzacio: value.localitzacio
        };
        (lavabo_dto,value.imatges, value.etiquetes)
    }
}

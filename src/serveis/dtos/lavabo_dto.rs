use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{dades::models::{lavabo::{Lavabo, LavaboAmbEtiquetes}}, routes::extractors::lavabo_extractors::CreateLavaboRequest};

use super::etiqueta_dto::EtiquetaDTO;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct LavaboDTO {
    pub(crate) id: Uuid,
    pub(crate) descripcio: String,
    pub(crate) titol: String,
    pub(crate) puntuacio_mitja: f32,
    pub(crate) nombre_resenyes: i64,
    pub(crate) created_at: DateTime<Utc>
}


impl From<Lavabo> for LavaboDTO {
    fn from(value: Lavabo) -> Self {
        Self { id: value.id, descripcio: value.descripcio, puntuacio_mitja: value.puntuacio_mitja, created_at: value.created_at, titol: value.titol, nombre_resenyes: value.nombre_resenyes}
    }
}

impl From<CreateLavaboRequest> for LavaboDTO {
    fn from(value: CreateLavaboRequest) -> Self {
        Self { id: Uuid::new_v4(), descripcio: value.descripcio.clone(), titol: value.titol.clone(), puntuacio_mitja: 0.0, nombre_resenyes: 0, created_at: Utc::now() }
    }
}

#[derive(Serialize,Deserialize, Debug)]
pub(crate) struct LavaboAmbEtiquetesDTO {
    pub(crate) id: Uuid,
    pub(crate) descripcio: String,
    pub(crate) titol: String,
    pub(crate) puntuacio_mitja: f32,
    pub(crate) nombre_resenyes: i64,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) etiquetes: Vec<EtiquetaDTO>
}

impl From<LavaboAmbEtiquetes> for LavaboAmbEtiquetesDTO {
    fn from(value: LavaboAmbEtiquetes) -> Self {
        Self { 
            id: value.id, 
            descripcio: value.descripcio, 
            titol: value.titol, 
            puntuacio_mitja: value.puntuacio_mitja, 
            nombre_resenyes: value.nombre_resenyes, 
            created_at: value.created_at, 
            etiquetes: value.etiquetes.into_iter().map(Into::into).collect()
        }
    }
}
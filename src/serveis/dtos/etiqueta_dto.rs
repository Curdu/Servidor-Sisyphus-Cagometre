use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{dades::models::etiqueta::Etiqueta, routes::extractors::etiqueta_extractors::CreateEtiquetaRequest};

#[derive(Serialize,Deserialize,Debug)]
pub(crate) struct EtiquetaDTO{
    pub(crate) id: Uuid,
    pub(crate) nom: String,
    pub(crate) created_at: DateTime<Utc>
}

impl From<Etiqueta> for EtiquetaDTO {
    fn from(value: Etiqueta) -> Self {
        EtiquetaDTO { id: value.id, nom: value.nom, created_at: value.created_at}
    }
}

impl From<CreateEtiquetaRequest> for EtiquetaDTO {
    fn from(value: CreateEtiquetaRequest) -> Self {
        EtiquetaDTO { id: Uuid::new_v4(), nom: value.nom, created_at: Utc::now() }
    }
}
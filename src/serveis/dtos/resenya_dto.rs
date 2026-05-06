use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{dades::models::resenya::{Resenya, ResenyaAmbPerfil}, routes::extractors::resenya_extractors::CreateResenyaRequest};
#[derive(Debug,Serialize,Deserialize)]
pub(crate) struct ResenyaDTO {
    pub(crate) id: Uuid,
    pub(crate) id_lavabo: Uuid,
    pub(crate) id_usuari: Uuid,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) comentari: String,
    pub(crate) puntuacio: f32
}

impl From<Resenya> for ResenyaDTO {
    fn from(value: Resenya) -> Self {
        ResenyaDTO { id: value.id, id_lavabo: value.id_lavabo, id_usuari: value.id_usuari, created_at: value.created_at, comentari: value.comentari, puntuacio: value.puntuacio }
    }
}

impl From<CreateResenyaRequest> for ResenyaDTO {
    fn from(value: CreateResenyaRequest) -> Self {
        Self { id: Uuid::new_v4(), id_lavabo: value.id_lavabo, id_usuari: value.id_usuari.unwrap(), created_at: Utc::now(), comentari: value.comentari, puntuacio: value.puntuacio }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub (crate) struct ResenyaAmbPerfilDTO {
    pub(crate) id: Uuid,
    pub(crate) id_lavabo: Uuid,
    pub(crate) id_usuari: Uuid,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) comentari: String,
    pub(crate) puntuacio: f32,
    pub(crate) nom_usuari: String,
    pub(crate) imatge_perfil: Option<String>
}
impl From<ResenyaAmbPerfil> for ResenyaAmbPerfilDTO {
    fn from(value: ResenyaAmbPerfil) -> Self {
        let imatge = match value.imatge_perfil {
            Some(i) => Some(format!("https://gcpxeolootoyuomnrpfu.supabase.co/storage/v1/object/public/perfil_images/{}",i)),
            None => None
        };
        Self { 
            id: value.id, 
            id_lavabo: value.id_lavabo, 
            id_usuari: value.id_usuari, 
            created_at: value.created_at, 
            comentari: value.comentari, 
            puntuacio: value.puntuacio, 
            nom_usuari: value.nom_usuari, 
            imatge_perfil:imatge 
        }
    }
}
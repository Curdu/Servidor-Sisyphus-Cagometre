use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::{dades::models::resenya::Resenya, serveis::dtos::lavabo_dto::LavaboDTO};

use super::etiqueta::Etiqueta;

#[derive(Debug,Serialize, Deserialize, FromRow)]
pub(crate) struct Lavabo {
    pub(crate) id: Uuid,
    pub(crate) descripcio: String,
    pub(crate) titol: String,
    pub(crate) puntuacio_mitja: f32,
    pub(crate) nombre_resenyes: i64,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) creador_id: Uuid
}

impl From<LavaboDTO> for Lavabo {
    fn from(value: LavaboDTO) -> Self {
        Self { id: value.id, descripcio: value.descripcio, puntuacio_mitja: value.puntuacio_mitja, created_at: value.created_at, titol: value.titol, nombre_resenyes: value.nombre_resenyes, creador_id: value.creador_id }
    }
}
#[derive(Debug,FromRow)]
pub(crate) struct LavaboAmbEtiquetes {
    pub(crate) id: Uuid,
    pub(crate) descripcio: String,
    pub(crate) titol: String,
    pub(crate) puntuacio_mitja: f32,
    pub(crate) nombre_resenyes: i64,
    pub(crate) created_at: DateTime<Utc>,
    #[sqlx(json)]
    pub(crate) etiquetes: Vec<Etiqueta>,
    #[sqlx(json)]
    pub(crate) imatges: Vec<LavaboImatge>,
    pub(crate) creador_id: Uuid
}

#[derive(FromRow, Debug, Deserialize)]
pub(crate) struct LavaboImatge {
    pub(crate) path: String,
    pub(crate) lavabo_id: Uuid,
    pub(crate) created_at: DateTime<Utc>
}

impl LavaboImatge {
    pub fn new (path: String, lavabo_id: Uuid, created_at : DateTime<Utc>) -> Self {
        Self{path,lavabo_id,created_at}
    }

    pub fn get_public_url (&self) -> String {
        format!("https://gcpxeolootoyuomnrpfu.supabase.co/storage/v1/object/public/lavabo_images/{}/{}", &self.lavabo_id, &self.path)
    }
}

#[derive(Debug,FromRow)]
pub(crate) struct LavaboDetallat {
    pub(crate) id: Uuid,
    pub(crate) descripcio: String,
    pub(crate) titol: String,
    pub(crate) puntuacio_mitja: f32,
    pub(crate) nombre_resenyes: i64,
    pub(crate) created_at: DateTime<Utc>,
    #[sqlx(json)]
    pub(crate) etiquetes: Vec<Etiqueta>,
    #[sqlx(json)]
    pub(crate) imatges: Vec<LavaboImatge>,
    pub(crate) creador_id: Uuid,
    #[sqlx(json)]
    pub(crate) resenyes: Vec<Resenya>    
}
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::dades::models::perfil::Perfil;

#[derive(Serialize,Deserialize,Debug)]
pub(crate) struct PerfilDTO {
    pub(crate) id: Uuid,
    pub(crate) nom: String,
    pub(crate) cognoms: String,
    pub(crate) imatge_url: Option<String>,
    pub(crate) nombre_de_resenyes: i64
}

impl From<Perfil> for PerfilDTO {
    fn from(value: Perfil) -> Self {
        let imatge = match value.imatge_url {
            Some(i) => Some(format!("https://gcpxeolootoyuomnrpfu.supabase.co/storage/v1/object/public/perfil_images/{}",i)),
            None => None
        };
        Self { id: value.id, nom: value.nom, cognoms: value.cognoms, imatge_url: imatge , nombre_de_resenyes: value.nombre_de_resenyes}
    }
}
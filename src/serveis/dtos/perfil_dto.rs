use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::dades::models::perfil::Perfil;

#[derive(Serialize,Deserialize,Debug)]
pub(crate) struct PerfilDTO {
    pub(crate) id: Uuid,
    pub(crate) nom: String,
    pub(crate) cognoms: String,
    pub(crate) imatge_url: Option<String>
}

impl From<Perfil> for PerfilDTO {
    fn from(value: Perfil) -> Self {
        Self { id: value.id, nom: value.nom, cognoms: value.cognoms, imatge_url: value.imatge_url }
    }
}
use axum_typed_multipart::{FieldData, TryFromMultipart};
use serde::{Deserialize, Serialize};
use tempfile::NamedTempFile;
use uuid::Uuid;

use crate::serveis::dtos::{perfil_dto::PerfilDTO, usuari_dto::UsuariDTO};

#[derive(Serialize, Deserialize)]
pub(crate) struct CrearUsuariRequest {
    pub(crate) correu: String,
    pub(crate) nom: String,
    pub(crate) cognoms: String,
    pub(crate) contrasenya: String
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ObtenirUsuariPerIdRequest {
    pub id: Uuid
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ActualitzarUsuariRequest {
    pub(crate) id: Uuid,
    pub(crate) usuari: UsuariDTO
}

#[derive(Serialize,Deserialize)]
pub(crate) struct EliminarUsuariRequest {
    pub(crate) id: Uuid
}

#[derive(Debug, TryFromMultipart)]

pub(crate) struct ModificarPerfilRequest {
    pub(crate) id: Uuid,
    pub(crate) nom: String,
    pub(crate) cognoms: String,
    #[form_data(limit= "unlimited")]
    pub(crate) imatge : Option<FieldData<NamedTempFile>>
}

impl From<ModificarPerfilRequest> for (Option<FieldData<NamedTempFile>>, PerfilDTO) {
    fn from(value: ModificarPerfilRequest) -> Self {
        let perfil_dto = PerfilDTO{id: value.id, nom: value.nom, cognoms: value.cognoms, imatge_url: None, nombre_de_resenyes: 0};
        let imatge = value.imatge;
        (imatge,perfil_dto)
    }
}
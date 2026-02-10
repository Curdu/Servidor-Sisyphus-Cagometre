use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::serveis::dtos::usuari_dto::UsuariDTO;

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
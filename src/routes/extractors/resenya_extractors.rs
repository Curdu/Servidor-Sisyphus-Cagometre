use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug,Serialize,Deserialize)]
pub struct CreateResenyaRequest {
    pub(crate) id_lavabo : Uuid,
    pub(crate) comentari: String,
    pub(crate) puntuacio: f32,
    pub(crate) id_usuari: Option<Uuid>
}
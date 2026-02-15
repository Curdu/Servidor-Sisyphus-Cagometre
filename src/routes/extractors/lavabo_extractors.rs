use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct CreateLavaboRequest {
    pub(crate) descripcio: String,
    pub(crate) puntuacio: f64,
}

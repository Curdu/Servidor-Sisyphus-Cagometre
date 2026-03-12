use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct CreateLavaboRequest {
    pub(crate) titol: String,
    pub(crate) descripcio: String,
}

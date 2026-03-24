use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct CreateEtiquetaRequest {
    pub(crate) nom: String,
}
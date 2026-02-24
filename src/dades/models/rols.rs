use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Type,Debug,PartialEq,Clone, Serialize, Deserialize)]
#[sqlx(type_name = "rols")]
pub(crate) enum UsuariRol{
    USUARI,
    ADMIN
}
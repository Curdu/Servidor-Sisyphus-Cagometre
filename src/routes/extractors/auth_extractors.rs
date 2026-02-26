use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize,Deserialize,Debug,Clone)]
pub(crate) struct ClaimsInfo {
    pub(crate) user_id: Uuid,
    pub(crate) exp: u64,
    pub(crate) correu: String
}
#[derive(Serialize,Deserialize,Debug,Clone)]
pub(crate) struct LoginRequest{
    pub(crate) correu: String,
    pub(crate) passwd: String
}
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize,Deserialize,Debug,Clone)]
pub(crate) struct ClaimsInfo {
    pub(crate) sub: String,
    pub(crate) exp: u64,
    pub(crate) email: String,
    pub(crate) iat: u64,
    pub(crate) aud: String,
    pub(crate) role: String,
    pub(crate) iss: String,
    pub(crate) app_metadata: Value,
    pub(crate) user_metadata: Value,
}
#[derive(Serialize,Deserialize,Debug,Clone)]
pub(crate) struct LoginRequest{
    pub(crate) correu: String,
    pub(crate) passwd: String
}
use serde::{Deserialize, Serialize};

use crate::routes::extractors::auth_extractors::ClaimsInfo;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub(crate) struct AuthToken {
    pub(crate) token : String
}

impl AuthToken {
    pub(crate) fn new(token :String) -> Self {
        Self { token }
    }
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub(crate) struct AuthDataDTO  {
    pub(crate) sub: String,
    pub(crate) exp: u64,
    pub(crate) correu: String,
    pub(crate) iat: u64,
    pub(crate) aud: String,
    pub(crate) role: String,
    pub(crate) token: String,
}

impl AuthDataDTO {
    pub fn new (claims: ClaimsInfo, token: String) -> Self {
        Self { sub:claims.sub, exp: claims.exp, correu: claims.email, iat: claims.iat, aud: claims.aud, role: claims.role, token }
    }
}
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct AuthToken {
    token : String
}

impl AuthToken {
    pub(crate) fn new(token :String) -> Self {
        Self { token }
    }
}
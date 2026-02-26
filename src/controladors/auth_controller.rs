use std::sync::Arc;

use async_trait::async_trait;

use crate::{errors::{auth_errors::AuthError, usuari_errors::UsuariErrors}, serveis::{auth_service::AuthService, dtos::{auth_dto::AuthToken, usuari_dto::UsuariDTO}}};

#[async_trait]
pub(crate) trait AuthController: Send + Sync {
    async fn login(&self, correu: String, passwd: String) -> Result<AuthToken, AuthError>;
    async fn registre(&self, usuari_dto: UsuariDTO) -> Result<(), UsuariErrors>;
}


pub(crate) struct AuthControlador {
    auth_service : Arc<dyn AuthService> 
}

impl AuthControlador {
    pub(crate) fn new(auth_service: Arc<dyn AuthService>) -> AuthControlador{
        Self{auth_service}
    }
}

#[async_trait]
impl AuthController for AuthControlador {

    async fn login(&self, correu: String, passwd: String) -> Result<AuthToken, AuthError> {
        self.auth_service.login(correu, passwd).await
    }
    async fn registre(&self, usuari_dto: UsuariDTO) -> Result<(), UsuariErrors> {
        self.auth_service.registre(usuari_dto.into()).await
    }
}
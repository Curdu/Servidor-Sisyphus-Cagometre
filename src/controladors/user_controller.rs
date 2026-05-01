use std::{ sync::Arc};

use async_trait::async_trait;
use uuid::Uuid;

use crate::{errors::usuari_errors::UsuariErrors, serveis::{dtos::{auth_dto::AuthDataDTO, perfil_dto::PerfilDTO, usuari_dto::UsuariDTO}, user_service::UserService}};



#[async_trait]
pub(crate) trait UserController: Send + Sync {
    async fn obte_usuari_per_id(&self ,id: Uuid) -> Result<UsuariDTO, UsuariErrors>;
    async fn actualitzar_usuari(&self ,id: Uuid,usuari_dto: UsuariDTO) -> Result<UsuariDTO, UsuariErrors>;
    async fn eliminar_usuari(&self ,id: Uuid) -> Result<(), UsuariErrors>;
    async fn crear_usuari(&self, usuari_dto: UsuariDTO) -> Result<(), UsuariErrors>;
    async fn obte_perfil_per_id(&self, id: Uuid, usr_data: AuthDataDTO) -> Result<PerfilDTO, UsuariErrors>;
}


pub(crate) struct UserControlador {
    usuari_service : Arc<dyn UserService>
}
impl UserControlador {
    pub(crate) fn new (usuari_servei : Arc<dyn UserService>) -> Self {
        Self { usuari_service:usuari_servei }
    }
}

#[async_trait]
impl UserController for UserControlador {

    async fn obte_usuari_per_id(&self ,id: Uuid) -> Result<UsuariDTO, UsuariErrors>{
        self.usuari_service.obtenir_usuari_per_id(id).await
    }
    async fn actualitzar_usuari(&self ,id: Uuid,usuari_dto: UsuariDTO) -> Result<UsuariDTO, UsuariErrors>{
        if id != usuari_dto.id {
            Err(UsuariErrors::NotEnoughtPermission("No pot modificar la informació d'un usuari diferent a el teu".to_string()))
        }else {
            self.usuari_service.actualitzar_usuari_amb_retorn(id, usuari_dto).await
        }
    }
    async fn eliminar_usuari(&self ,id: Uuid) -> Result<(), UsuariErrors>{
        self.usuari_service.eliminar_usuari(id).await
    }
    async fn crear_usuari(&self, usuari_dto: UsuariDTO) -> Result<(), UsuariErrors> {
        self.usuari_service.crear_usuari(usuari_dto).await
    }
    async fn obte_perfil_per_id(&self, id: Uuid, usr_data: AuthDataDTO) -> Result<PerfilDTO, UsuariErrors> {
        
        if usr_data.sub != id.to_string() {
            return Err(UsuariErrors::NotEnoughtPermission("No es pot accedir al perfil d'un altre usuari".to_string()))
        }
        self.usuari_service.obtenir_perfil_per_id(id).await
    }

}


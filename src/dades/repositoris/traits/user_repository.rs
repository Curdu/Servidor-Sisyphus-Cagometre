

use async_trait::async_trait;
use uuid::Uuid;

use crate::{dades::models::{perfil::Perfil, usuari::Usuari}, errors::usuari_errors::UsuariErrors};

#[async_trait]
pub(crate)  trait UserRepository: Sync + Send {
    async fn obte_usuari_per_id(&self ,id: Uuid) -> Result<Usuari,UsuariErrors>;
    async fn obte_usuari_per_correu(&self ,correu: String) -> Result<Usuari, UsuariErrors>;
    async fn crear_usuari(&self ,usuari: Usuari) -> Result<(),UsuariErrors>;
    async fn actualitza_usuari(&self ,id: Uuid, nou_usuari: Usuari) -> Result<(),UsuariErrors>;
    async fn elimina_usuari(&self ,id: Uuid) -> Result<(), UsuariErrors>;
    async fn obtenir_perfil_per_id(&self, id: Uuid) -> Result<Perfil, UsuariErrors>;
}
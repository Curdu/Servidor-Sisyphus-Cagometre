use async_trait::async_trait;
use uuid::Uuid;

use crate::{dades::models::lavabo::LavaboImatge, errors::crud_errors::CrudErrors};

#[async_trait]
pub (crate) trait LavaboImatgeRepository: Send + Sync {
    async fn obtenir_imatges_per_lavabo_id(&self, lavabo_id: Uuid) -> Result<Vec<LavaboImatge>, CrudErrors>;
    async fn crear_imate_lavabo(&self, lavabo_imatge : LavaboImatge) -> Result<(), CrudErrors>;
    async fn eliminar_imatge_lavabo(&self, lavabo_id: Uuid, path : String) -> Result<(), CrudErrors>;
}
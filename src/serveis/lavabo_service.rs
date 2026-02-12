use async_trait::async_trait;
use uuid::Uuid;

use crate::{errors::lavabo_errors::LavaboErrors, serveis::dtos::lavabo_dto::LavaboDTO};

#[async_trait]
pub(crate) trait LavaboService: Sync + Send {
    async fn crear_lavabo(&self ,lavabo_dto: LavaboDTO) -> Result<LavaboDTO, LavaboErrors>;
    async fn obte_lavabo_per_id(&self, id: Uuid) -> Result<LavaboDTO, LavaboErrors>;
    async fn actualitzar_lavabo(&self, id: Uuid, lavabo_dto: LavaboDTO) -> Result<LavaboDTO, LavaboErrors>;
    async fn eliminar_lavabo(&self, id: Uuid) -> Result<(), LavaboErrors>;
}
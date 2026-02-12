use async_trait::async_trait;
use uuid::Uuid;

use crate::{dades::models::lavabo::Lavabo, errors::lavabo_errors::LavaboErrors};

#[async_trait]
pub(crate) trait LavaboRepository: Sync + Send {
    async fn obte_lavabo_per_id(id: Uuid) -> Result<Lavabo, LavaboErrors>;
    async fn crear_lavabo(lavabo : Lavabo) -> Result<Lavabo, LavaboErrors>;
    async fn actualitzar_lavabo(id: Uuid, lavabo : Lavabo) -> Result<Lavabo, LavaboErrors>;
    async fn eliminar_lavabo(id: Uuid) -> Result<(), LavaboErrors>;
    
}
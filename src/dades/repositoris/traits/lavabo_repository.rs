use async_trait::async_trait;
use uuid::Uuid;

use crate::{dades::models::lavabo::{Lavabo, LavaboAmbEtiquetes, LavaboDetallat}, errors::lavabo_errors::LavaboErrors};

#[async_trait]
pub(crate) trait LavaboRepository: Sync + Send {
    async fn obte_lavabo_per_id(&self,id: Uuid) -> Result<Lavabo, LavaboErrors>;
    async fn crear_lavabo(&self,lavabo : Lavabo) -> Result<(), LavaboErrors>;
    async fn actualitzar_lavabo(&self,id: Uuid, lavabo : Lavabo) -> Result<(), LavaboErrors>;
    async fn eliminar_lavabo(&self,id: Uuid) -> Result<(), LavaboErrors>;
    async fn obte_tots_lavabos(&self) -> Result<Vec<Lavabo>, LavaboErrors>;
    async fn obte_tots_lavabos_amb_etiquetes(&self) -> Result<Vec<LavaboAmbEtiquetes>, LavaboErrors>;
    async fn obte_lavabo_detallat_per_id(&self, id: Uuid) -> Result<LavaboDetallat, LavaboErrors>; 
    
}
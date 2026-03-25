use std::{sync::Arc};

use async_trait::async_trait;
use uuid::Uuid;

use crate::{dades::repositoris::traits::lavabo_repository::{LavaboRepository}, errors::lavabo_errors::LavaboErrors, serveis::dtos::lavabo_dto::LavaboDTO};

use super::dtos::lavabo_dto::LavaboAmbEtiquetesDTO;

#[async_trait]
pub(crate) trait LavaboService: Sync + Send {
    async fn crear_lavabo(&self ,lavabo_dto: LavaboDTO) -> Result<(), LavaboErrors>;
    async fn obte_lavabo_per_id(&self, id: Uuid) -> Result<LavaboDTO, LavaboErrors>;
    async fn actualitzar_lavabo(&self, id: Uuid, lavabo_dto: LavaboDTO) -> Result<LavaboDTO, LavaboErrors>;
    async fn eliminar_lavabo(&self, id: Uuid) -> Result<(), LavaboErrors>;
    async fn obte_tots_lavabos(&self) -> Result<Vec<LavaboDTO>, LavaboErrors>;
    async fn obte_tots_lavabos_amb_etiquetes(&self) -> Result<Vec<LavaboAmbEtiquetesDTO>, LavaboErrors>;
}

pub(crate) struct  LavaboServei {
    lavabo_repository : Arc<dyn LavaboRepository>
}

impl LavaboServei {
    pub(crate) fn new(lavabo_repository: Arc<dyn LavaboRepository>) -> Self {
        Self { lavabo_repository }
    }

}

#[async_trait]
impl LavaboService for LavaboServei {
    async fn crear_lavabo(&self ,lavabo_dto: LavaboDTO) -> Result<(), LavaboErrors>{
        self.lavabo_repository.crear_lavabo(lavabo_dto.into()).await
    }


    async fn obte_lavabo_per_id(&self, id: Uuid) -> Result<LavaboDTO, LavaboErrors>{
        let result = self.lavabo_repository.obte_lavabo_per_id(id).await;

        match result {
            Ok(lavabo) => {
                Ok(lavabo.into())
            },
            Err(error) => Err(error)
        }
    }

    async fn actualitzar_lavabo(&self, id: Uuid, lavabo_dto: LavaboDTO) -> Result<LavaboDTO, LavaboErrors>{
        let result = self.lavabo_repository.actualitzar_lavabo(id.clone(), lavabo_dto.into()).await;

        match result {
            Ok(()) => {
                let lavabo = self.lavabo_repository.obte_lavabo_per_id(id).await;
                match lavabo {
                    Ok(lvb) => Ok(lvb.into()),
                    Err(error) => Err(error)
                }
            },
            Err(error) => Err(error)
        }
    }
    async fn eliminar_lavabo(&self, id: Uuid) -> Result<(), LavaboErrors>{
        self.lavabo_repository.eliminar_lavabo(id).await
    }

    async fn obte_tots_lavabos(&self) -> Result<Vec<LavaboDTO>, LavaboErrors> {
        let lavabos = self.lavabo_repository.obte_tots_lavabos().await?;
        Ok(lavabos.into_iter().map(Into::into).collect())
    }
    async fn obte_tots_lavabos_amb_etiquetes(&self) -> Result<Vec<LavaboAmbEtiquetesDTO>, LavaboErrors>{
        let lavabos_amb_etiquetes = self.lavabo_repository.obte_tots_lavabos_amb_etiquetes().await?;
        Ok(lavabos_amb_etiquetes.into_iter().map(Into::into).collect())        
    }
    
}
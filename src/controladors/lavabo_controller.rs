use async_trait::async_trait;
use axum_typed_multipart::FieldData;
use tempfile::NamedTempFile;
use std::sync::Arc;
use uuid::Uuid;

use crate::{errors::lavabo_errors::LavaboErrors, serveis::{dtos::{auth_dto::AuthDataDTO, lavabo_dto::{LavaboAmbEtiquetesDTO, LavaboDTO}}, lavabo_service::LavaboService}};

#[async_trait]
pub(crate) trait LavaboController: Sync + Send {
    async fn crear_lavabo(&self, lavabo_dto: LavaboDTO, imatges: Vec<FieldData<NamedTempFile>>, auth_data : AuthDataDTO) -> Result<(), LavaboErrors>;
    async fn get_lavabo_per_id(&self,id: Uuid) -> Result<LavaboDTO, LavaboErrors>;
    async fn actualitzar_lavabo(&self,id: Uuid, lavabo_dto: LavaboDTO) -> Result<LavaboDTO, LavaboErrors>;
    async fn eliminar_lavabo(&self,id: Uuid) -> Result<(), LavaboErrors>;
    async fn get_tots_lavabos(&self) -> Result<Vec<LavaboDTO>, LavaboErrors>;
    async fn get_tots_lavabos_amb_etiqueta(&self) -> Result<Vec<LavaboAmbEtiquetesDTO>, LavaboErrors>;
}

pub(crate) struct LavaboControlador {
    lavabo_service : Arc<dyn LavaboService> 
}

impl LavaboControlador {
    pub(crate) fn new(lavabo_service : Arc<dyn LavaboService>) -> Self {
        Self { lavabo_service }
    }
}

#[async_trait]
impl LavaboController for LavaboControlador {
    async fn crear_lavabo(&self, lavabo_dto: LavaboDTO, imatges: Vec<FieldData<NamedTempFile>>, auth_data : AuthDataDTO) -> Result<(), LavaboErrors>{
        self.lavabo_service.crear_lavabo(lavabo_dto, imatges, auth_data).await
    }
    async fn get_lavabo_per_id(&self,id: Uuid) -> Result<LavaboDTO, LavaboErrors>{
        self.lavabo_service.obte_lavabo_per_id(id).await
    }
    async fn actualitzar_lavabo(&self,id: Uuid, lavabo_dto: LavaboDTO) -> Result<LavaboDTO, LavaboErrors>{
        self.lavabo_service.actualitzar_lavabo(id, lavabo_dto).await
    }
    async fn eliminar_lavabo(&self,id: Uuid) -> Result<(), LavaboErrors>{
        self.lavabo_service.eliminar_lavabo(id).await
    }
    async fn get_tots_lavabos(&self) -> Result<Vec<LavaboDTO>, LavaboErrors> {
        self.lavabo_service.obte_tots_lavabos().await
    }
    async fn get_tots_lavabos_amb_etiqueta(&self) -> Result<Vec<LavaboAmbEtiquetesDTO>, LavaboErrors> {
        self.lavabo_service.obte_tots_lavabos_amb_etiquetes().await
    }
}
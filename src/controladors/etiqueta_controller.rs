use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{errors::crud_errors::CrudErrors, serveis::{dtos::etiqueta_dto::EtiquetaDTO, iservice::IService}};

use super::icontroller::IController;

pub(crate) struct EtiquetaControlador {
    etiqueta_service : Arc<dyn IService<EtiquetaDTO>>
}
impl EtiquetaControlador {
    pub fn new (etiqueta_service : Arc<dyn IService<EtiquetaDTO>>) -> Self {
        Self { etiqueta_service }
    }
}

#[async_trait]
impl IController<EtiquetaDTO> for EtiquetaControlador {

    async fn crear(&self, dto: EtiquetaDTO) -> Result<(), CrudErrors> {
        self.etiqueta_service.crear(dto).await
    }
    async fn get_per_id(&self,id: Uuid) -> Result<EtiquetaDTO, CrudErrors> {
        self.etiqueta_service.obte_per_id(id).await
    }
    async fn actualitzar(&self,id: Uuid, dto: EtiquetaDTO) -> Result<EtiquetaDTO, CrudErrors> {
        self.etiqueta_service.actualitzar(id, dto).await
    }
    async fn eliminar(&self,id: Uuid) -> Result<(), CrudErrors> {
        self.etiqueta_service.eliminar(id).await
    }

}
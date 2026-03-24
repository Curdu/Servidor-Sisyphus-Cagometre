use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{errors::crud_errors::CrudErrors, serveis::{dtos::resenya_dto::ResenyaDTO, iservice::IService}};

use super::icontroller::IController;

pub(crate) struct ResenyaControlador {
    resenya_service : Arc<dyn IService<ResenyaDTO>>
}

impl ResenyaControlador {
    pub fn new (resenya_service: Arc<dyn IService<ResenyaDTO>>) -> Self {
        Self { resenya_service }
    }
}

#[async_trait]
impl IController<ResenyaDTO> for ResenyaControlador {

    async fn crear(&self, dto: ResenyaDTO) -> Result<(), CrudErrors> {
        self.resenya_service.crear(dto).await
    }
    async fn get_per_id(&self,id: Uuid) -> Result<ResenyaDTO, CrudErrors> {
        self.resenya_service.obte_per_id(id).await
    }
    async fn actualitzar(&self,id: Uuid, dto: ResenyaDTO) -> Result<ResenyaDTO, CrudErrors> {
        self.resenya_service.actualitzar(id, dto).await
    }
    async fn eliminar(&self,id: Uuid) -> Result<(), CrudErrors> {
        self.resenya_service.eliminar(id).await
    }

}
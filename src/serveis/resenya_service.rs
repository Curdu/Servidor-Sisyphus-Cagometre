use std::sync::Arc;

use async_trait::async_trait;


use crate::dades::{models::resenya::Resenya, repositoris::traits::compartit::irepository::IRepository};
use crate::errors::crud_errors::{CrudErrors};
use super::{dtos::resenya_dto::ResenyaDTO, iservice::IService};

pub(crate) struct ResenyaServei {
    resenya_repository : Arc<dyn IRepository<Resenya>>
}

#[async_trait]
impl IService<ResenyaDTO> for ResenyaServei {
    async fn crear(&self ,resenya_dto: ResenyaDTO) -> Result<(), CrudErrors> {
        self.resenya_repository.crear(resenya_dto.into()).await
    }

    async fn obte_per_id(&self, id: uuid::Uuid) -> Result<ResenyaDTO, CrudErrors> {
        let result = self.resenya_repository.obte_per_id(id).await;
        Ok(result?.into())
    }

    async fn actualitzar(&self, id: uuid::Uuid, resenya_dto: ResenyaDTO) -> Result<ResenyaDTO, CrudErrors> {
        self.resenya_repository.actualitzar(id, resenya_dto.into()).await?;
        let resenya = self.obte_per_id(id).await?;
        Ok(resenya.into())
    }

    async fn eliminar(&self, id: uuid::Uuid) -> Result<(), CrudErrors> {
        self.resenya_repository.eliminar(id).await
    }
}
impl ResenyaServei {
    pub fn new(resenya_repository: Arc<dyn IRepository<Resenya>>) -> Self {
        Self { resenya_repository: resenya_repository }
    }
}
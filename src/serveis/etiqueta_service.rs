use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{dades::{models::etiqueta::Etiqueta, repositoris::compartit::irepository::IRepository}, errors::crud_errors::CrudErrors};

use super::{dtos::etiqueta_dto::EtiquetaDTO, iservice::IService};

pub(crate) struct EtiquetaServei {
    etiqueta_repository: Arc<dyn IRepository<Etiqueta>>
}

impl EtiquetaServei {
    pub fn new(etiqueta_repository: Arc<dyn IRepository<Etiqueta>>) -> Self {
        Self { etiqueta_repository }
    }
}

#[async_trait]
impl IService<EtiquetaDTO> for EtiquetaServei {

    async fn crear(&self ,dto: EtiquetaDTO) -> Result<(), CrudErrors>{
        self.etiqueta_repository.crear(dto.into()).await
    }
    async fn obte_per_id(&self, id: Uuid) -> Result<EtiquetaDTO, CrudErrors>{
        let etiqueta = self.etiqueta_repository.obte_per_id(id).await?;
        Ok(etiqueta.into())
    }
    async fn actualitzar(&self, id: Uuid, dto: EtiquetaDTO) -> Result<EtiquetaDTO, CrudErrors>{
        self.etiqueta_repository.actualitzar(id, dto.into()).await?;
        let etiqueta = self.etiqueta_repository.obte_per_id(id).await?;
        Ok(etiqueta.into())

    }
    async fn eliminar(&self, id: Uuid) -> Result<(), CrudErrors>{
        self.etiqueta_repository.eliminar(id).await
    }
}
use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{dades::{repositoris::traits::{ etiqueta_repository::EtiquetaRepository}}, errors::crud_errors::CrudErrors};

use super::{dtos::etiqueta_dto::EtiquetaDTO, iservice::IService};

#[async_trait]
pub (crate) trait EtiquetaService: IService<EtiquetaDTO> {
    async fn obte_totes_etiquetes(&self) -> Result<Vec<EtiquetaDTO>, CrudErrors>;
}

pub(crate) struct EtiquetaServei {
    etiqueta_repository: Arc<dyn EtiquetaRepository>
}

impl EtiquetaServei {
    pub fn new(etiqueta_repository: Arc<dyn EtiquetaRepository>) -> Self {
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

#[async_trait]
impl EtiquetaService for EtiquetaServei {
    async fn obte_totes_etiquetes(&self) -> Result<Vec<EtiquetaDTO>, CrudErrors>{
        Ok(self.etiqueta_repository.obte_totes_etiquetes().await?.into_iter().map(Into::into).collect()) 
    }
}
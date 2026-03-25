use async_trait::async_trait;

use crate::{dades::models::etiqueta::Etiqueta, errors::crud_errors::CrudErrors};

use super::compartit::irepository::IRepository;

#[async_trait]
pub(crate) trait EtiquetaRepository: IRepository<Etiqueta> {
    async fn obte_totes_etiquetes(&self) -> Result<Vec<Etiqueta>, CrudErrors>;
}
use async_trait::async_trait;
use uuid::Uuid;

use crate::errors::crud_errors::CrudErrors;

#[async_trait]
pub(crate) trait IRepository<T>: Sync + Send {
    async fn obte_per_id(&self,id: Uuid) -> Result<T, CrudErrors>;
    async fn crear(&self,model : T) -> Result<(), CrudErrors>;
    async fn actualitzar(&self,id: Uuid, model : T) -> Result<(), CrudErrors>;
    async fn eliminar(&self,id: Uuid) -> Result<(), CrudErrors>;

}
use async_trait::async_trait;
use uuid::Uuid;

use crate::errors::crud_errors::CrudErrors;

#[async_trait]
pub trait IController<T>: Send + Sync {
    async fn crear(&self, dto: T) -> Result<(), CrudErrors>;
    async fn get_per_id(&self,id: Uuid) -> Result<T, CrudErrors>;
    async fn actualitzar(&self,id: Uuid, dto: T) -> Result<T, CrudErrors>;
    async fn eliminar(&self,id: Uuid) -> Result<(), CrudErrors>;
    
}
use async_trait::async_trait;

use crate::{dades::models::imatge::StorageImatge, errors::storage_errors::StorageError, serveis::dtos::auth_dto::AuthDataDTO};

#[async_trait]
pub(crate) trait ImatgesRepository: Send + Sync {

    async fn pujar_imatge(&self, auth_data: &AuthDataDTO, storage_imatge : StorageImatge) -> Result<(), StorageError>;
    async fn eliminar_imatges(&self, auth_data: &AuthDataDTO, bucket_id: String, imatges_paths : Vec<String>) -> Result<(), StorageError>;
}
use std::{sync::Arc};

use async_trait::async_trait;
use reqwest::StatusCode;
use tokio::sync::Mutex;
use tokio::fs::File;
use tokio_util::io::ReaderStream;
use serde_json::json;

use crate::{dades::{models::imatge::StorageImatge, repositoris::traits::image_repository::ImatgesRepository}, errors::storage_errors::StorageError, serveis::dtos::auth_dto::AuthDataDTO, state::{SUPABASE_API_KEY, SUPABASE_URL}};



pub(crate) struct SupabaseImatgesRepository {
    pub(crate) client : Arc<Mutex<reqwest::Client>> 
}

 #[async_trait]
impl ImatgesRepository for SupabaseImatgesRepository {
    async fn pujar_imatge(&self, auth_data: &AuthDataDTO, storage_imatge : StorageImatge ) -> Result<(), StorageError> {

        let mime_type_default = "image/jpeg".to_string();

        let tipo_contenido = storage_imatge.imatge.metadata.content_type.as_ref().unwrap_or(&mime_type_default);

        let ruta_temporal = storage_imatge.imatge.contents.path();
        let file = File::open(ruta_temporal).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR).unwrap();
        
        let stream = ReaderStream::new(file);
        let body = reqwest::Body::wrap_stream(stream);

        let upload_url = get_imatge_url(&storage_imatge);

        let response = self.client.lock().await.post(&upload_url)
           .header("apikey", &*SUPABASE_API_KEY)
           .header("Authorization", format!("Bearer {}", auth_data.token))
           .header("Content-Type", tipo_contenido)
           .body(body)
           .send()
           .await;
        
        match response {
            Ok(resp) => {
                if!resp.status().is_success() {
                    let error = resp.text().await.unwrap_or_default();
                    return Err(StorageError::ServerError(format!("Error al intentar puja la imatge. Error: {}", error)));
                }
                return Ok(())
            },
            Err(error) => {
                return Err(StorageError::ServerError(error.to_string()));
            }
        }

    }

    async fn eliminar_imatges(&self, auth_data: &AuthDataDTO, bucket_id: String, imatges_paths : Vec<String>) -> Result<(), StorageError>{
        let url = format!("{}/storage/v1/object/{}",&*SUPABASE_URL ,bucket_id);
        

        let imatges_a_eliminar = json!({
            "prefixes": imatges_paths
        });

        let response = self.client.lock().await.
        delete(&url)
           .header("apikey", &*SUPABASE_API_KEY)
           .header("Authorization", format!("Bearer {}", auth_data.token))
           .json(&imatges_a_eliminar)
           .send()
           .await;

        match response {
            Ok(resp) => {
                if!resp.status().is_success() {
                    let error = resp.text().await.unwrap_or_default();
                    return Err(StorageError::ServerError(format!("Error al intentar eliminar la imatge. Error: {}", error)));
                }
                return Ok(())
            },
            Err(error) => {
                return Err(StorageError::ServerError(error.to_string()));
            }
        }

    }

}


fn get_imatge_url(storage_imatge : &StorageImatge) -> String {
    let directori_defaults = "".to_string();
    let directori = storage_imatge.directories_path.as_ref().unwrap_or(&directori_defaults);
    let nom_archiu_default = "sense_nom.jpg".to_string();
    let nombre_archivo = &storage_imatge.imatge.metadata.file_name.as_ref().unwrap_or(&nom_archiu_default);
    format!("{}/storage/v1/object/{}/{}{}", &*SUPABASE_URL, storage_imatge.bucket_id,directori, nombre_archivo)
}

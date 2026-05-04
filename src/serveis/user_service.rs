use std::{sync::Arc};

use async_trait::async_trait;
use axum_typed_multipart::FieldData;
use rand::{RngExt, distr::Alphanumeric};
use sha2::{Digest, Sha512};
use tempfile::NamedTempFile;
use uuid::Uuid;

use crate::{dades::{models::{imatge::{StorageImatge}, perfil::Perfil, usuari::Usuari}, repositoris::traits::{image_repository::ImatgesRepository, user_repository::UserRepository}}, errors::usuari_errors::UsuariErrors, serveis::dtos::{auth_dto::AuthDataDTO, perfil_dto::PerfilDTO, usuari_dto::UsuariDTO}};

#[async_trait]
pub(crate) trait UserService: Send + Sync {
    
    async fn obtenir_usuari_per_id(&self ,id: Uuid) -> Result<UsuariDTO, UsuariErrors>;
    async fn crear_usuari(&self ,usuari: UsuariDTO) -> Result<(),UsuariErrors>;
    async fn crear_usuari_amb_retorn(&self ,usuari: UsuariDTO) -> Result<UsuariDTO, UsuariErrors>;
    async fn actualitzar_usuari(&self ,id: Uuid, usuari: UsuariDTO) -> Result<(),UsuariErrors>;
    async fn actualitzar_usuari_amb_retorn(&self ,id: Uuid, usuari: UsuariDTO) -> Result<UsuariDTO, UsuariErrors>;
    async fn eliminar_usuari(&self ,id: Uuid) -> Result<(),UsuariErrors>;
    async fn obtenir_perfil_per_id(&self, id: Uuid) -> Result<PerfilDTO, UsuariErrors>;
    async fn modificar_perfil(&self, id: Uuid, perfil_dto : PerfilDTO, imatge: Option<FieldData<NamedTempFile>>,auth_data : AuthDataDTO) -> Result<PerfilDTO, UsuariErrors>;
}

pub(crate) struct UserServei {
    user_repository: Arc<dyn UserRepository>,
    images_repository: Arc<dyn ImatgesRepository>,

}

#[async_trait]
impl UserService for UserServei {

    async fn obtenir_usuari_per_id(&self ,id: Uuid) -> Result<UsuariDTO, UsuariErrors> {
        let result = self.user_repository.obte_usuari_per_id(id).await;
        match result {
            Ok(user) => {
                Ok(user.into())
            },
            Err(error) => {
                Err(error)
            }
        }
    }

    async fn crear_usuari(&self ,usuari_dto: UsuariDTO) -> Result<(),UsuariErrors> {
        let (hash, salt) = generar_hash(&usuari_dto.contrasenya);
        let mut usuari : Usuari = usuari_dto.into();
        usuari.hash = hash;
        usuari.salt = salt;
        usuari.id = Uuid::new_v4();
        self.user_repository.crear_usuari(usuari).await
        
    }
    async fn crear_usuari_amb_retorn(&self ,usuari_dto: UsuariDTO) -> Result<UsuariDTO, UsuariErrors> {
        let (hash, salt) = generar_hash(&usuari_dto.contrasenya);
        let mut usuari : Usuari = usuari_dto.into();
        usuari.hash = hash;
        usuari.salt = salt;
        let id = Uuid::new_v4();
        usuari.id = id.clone();
        
        let response = self.user_repository.crear_usuari(usuari).await;
        match response {
            Ok(_)=> {
                let result = self.user_repository.obte_usuari_per_id(id).await;
                match result {
                    Ok(user) => {
                        Ok(user.into())

                    },
                    Err(error) =>{
                        Err(error)
                    }
                }
            },
            Err(error) => Err(error)
            
        }

    }
    async fn actualitzar_usuari(&self ,id: Uuid, usuari_dto: UsuariDTO) -> Result<(),UsuariErrors> {
        let usuari: Usuari = usuari_dto.into();
        self.user_repository.actualitza_usuari(id, usuari).await
    }
    async fn actualitzar_usuari_amb_retorn(&self ,id: Uuid, usuari_dto: UsuariDTO) -> Result<UsuariDTO, UsuariErrors> {
        let usuari: Usuari = usuari_dto.into();
        let response = self.user_repository.actualitza_usuari(id, usuari).await;
        match response {
            Ok(_) =>{
                let result = self.user_repository.obte_usuari_per_id(id).await;
                match result {
                    Ok(user) => Ok(user.into()),
                    Err(error) => Err(error)
                }
            }Err(error) =>{
                Err(error)
            }
        }
        
    }
    async fn eliminar_usuari(&self ,id: Uuid) -> Result<(),UsuariErrors> {
        self.user_repository.elimina_usuari(id).await
    }
    async fn obtenir_perfil_per_id(&self, id: Uuid) -> Result<PerfilDTO, UsuariErrors> {
        let perfil = self.user_repository.obtenir_perfil_per_id(id).await?;

        Ok(perfil.into())
    }
    
    async fn modificar_perfil(&self, id: Uuid, perfil_dto : PerfilDTO, imatge: Option<FieldData<NamedTempFile>>, auth_data : AuthDataDTO) -> Result<PerfilDTO, UsuariErrors> {

        let mut perfil : Perfil = perfil_dto.into();
        println!("{:?}", imatge);
        if let Some(img) = imatge {
            let nom_imatge = img.metadata.clone().file_name.unwrap();
            let storage_imatge = StorageImatge{
                bucket_id: "perfil_images".to_string(), 
                directories_path: Some(format!("{}/", auth_data.sub)),
                imatge: img};
            let result = self.images_repository.pujar_imatge(&auth_data, storage_imatge).await;
            match result {
                Ok(()) => perfil.imatge_url = Some(format!("{}/{}",auth_data.sub, nom_imatge)),
                Err(err) => {
                    println!("{:?}", err);
                    return Err(UsuariErrors::ServerError("Error pujant la imatge del perfil".to_string()))
                }
            }
            
        }
        
        self.user_repository.actualitzar_perfil(id, perfil).await?;
        Ok(self.user_repository.obtenir_perfil_per_id(id).await?.into()) 
    }


}

impl UserServei {
    pub(crate) fn new(usuari_repository: Arc<dyn UserRepository>, images_repository: Arc<dyn ImatgesRepository>) -> Self {
        Self { user_repository: usuari_repository, images_repository }
    }
}

pub(crate) fn generar_hash(contrasenya: &String) -> (String, String) {
    let rng = rand::rng();
    let salt : String = rng.sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();

    let mut hasher = Sha512::new();

    hasher.update(salt.as_bytes());
    hasher.update(contrasenya.as_bytes());

    let result = hasher.finalize();

    let hash_hex = hex::encode(result);

    (hash_hex, salt)
}
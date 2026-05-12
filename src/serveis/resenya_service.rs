use std::sync::Arc;

use async_trait::async_trait;


use crate::dades::repositoris::traits::lavabo_repository::LavaboRepository;
use crate::dades::{models::resenya::Resenya, repositoris::traits::compartit::irepository::IRepository};
use crate::errors::crud_errors::{CrudErrors};
use super::{dtos::resenya_dto::ResenyaDTO, iservice::IService};

pub(crate) struct ResenyaServei {
    resenya_repository : Arc<dyn IRepository<Resenya>>,
    lavabo_repository: Arc<dyn LavaboRepository>
}

#[async_trait]
impl IService<ResenyaDTO> for ResenyaServei {
    async fn crear(&self ,resenya_dto: ResenyaDTO) -> Result<(), CrudErrors> {
        let puntuacio : f32 = resenya_dto.puntuacio;
        let id_lavabo = resenya_dto.id_lavabo.clone();
            self.resenya_repository.crear(resenya_dto.into()).await?;
        let result = self.lavabo_repository.obte_lavabo_per_id(id_lavabo).await;
        match result {
            Ok(mut lavabo) => {
                lavabo.puntuacio_mitja = (lavabo.nombre_resenyes as f32 *lavabo.puntuacio_mitja+puntuacio)/(lavabo.nombre_resenyes+1) as f32;
                lavabo.nombre_resenyes = lavabo.nombre_resenyes+1;
                let lavabo_result = self.lavabo_repository.modificar_puntuacio(lavabo.puntuacio_mitja,lavabo.nombre_resenyes,lavabo.id).await;
                match lavabo_result {
                    Ok(()) => Ok(()),
                    Err(_) => Err(CrudErrors::ServerError("Error actualitzant el lavabo".to_string()))
                }
            }
            Err(_) => Err(CrudErrors::ServerError("Error inesperat".to_string()))
        }

    }

    async fn obte_per_id(&self, id: uuid::Uuid) -> Result<ResenyaDTO, CrudErrors> {
        let result = self.resenya_repository.obte_per_id(id).await;
        Ok(result?.into())
    }

    async fn actualitzar(&self, id: uuid::Uuid, resenya_dto: ResenyaDTO) -> Result<ResenyaDTO, CrudErrors> {

        let resenya_antiga = self.resenya_repository.obte_per_id(id.clone()).await?;
        let lavabo = self.lavabo_repository.obte_lavabo_per_id(resenya_dto.id_lavabo).await.ok().unwrap();
        
        let nova_puntuacio = ((lavabo.puntuacio_mitja * lavabo.nombre_resenyes as f32) - resenya_antiga.puntuacio + resenya_dto.puntuacio)/lavabo.nombre_resenyes as f32;
        self.resenya_repository.actualitzar(id, resenya_dto.into()).await?;
        if let Err(_) = self.lavabo_repository.modificar_puntuacio(nova_puntuacio, lavabo.nombre_resenyes, lavabo.id).await{
            return Err(CrudErrors::ServerError("Error inesperat".to_string()))
        }
        let resenya = self.obte_per_id(id).await?;
        Ok(resenya.into())
    }

    async fn eliminar(&self, id: uuid::Uuid) -> Result<(), CrudErrors> {
        self.resenya_repository.eliminar(id).await
    }
}
impl ResenyaServei {
    pub fn new(resenya_repository: Arc<dyn IRepository<Resenya>>, lavabo_repository: Arc<dyn LavaboRepository>) -> Self {
        Self { resenya_repository: resenya_repository, lavabo_repository}
    }
}
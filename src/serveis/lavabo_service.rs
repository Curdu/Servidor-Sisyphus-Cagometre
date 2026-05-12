use std::{ collections::HashSet, sync::Arc};

use async_trait::async_trait;
use axum_typed_multipart::{FieldData};
use chrono::Utc;
use tempfile::NamedTempFile;
use uuid::Uuid;

use crate::{dades::{models::{etiqueta::Etiqueta, imatge::StorageImatge, lavabo::{Lavabo, LavaboDetallat, LavaboImatge}}, repositoris::traits::{ image_repository::ImatgesRepository, lavabo_imatge_repository::LavaboImatgeRepository, lavabo_repository::LavaboRepository}}, errors::{lavabo_errors::LavaboErrors, storage_errors::StorageError}, serveis::dtos::{etiqueta_dto::EtiquetaDTO, lavabo_dto::{LavaboDTO, LavaboDetallatDTO}}};

use super::dtos::{auth_dto::AuthDataDTO, lavabo_dto::LavaboAmbEtiquetesDTO};

#[async_trait]
pub(crate) trait LavaboService: Sync + Send {
    async fn crear_lavabo(&self ,lavabo_dto: LavaboDTO, imatges : Vec<FieldData<NamedTempFile>>, auth_data: AuthDataDTO, etiquetes: Vec<Uuid>) -> Result<(), LavaboErrors>;
    async fn obte_lavabo_per_id(&self, id: Uuid) -> Result<LavaboDTO, LavaboErrors>;
    async fn actualitzar_lavabo(&self, id: Uuid, lavabo_dto: LavaboDetallatDTO) -> Result<LavaboDetallatDTO, LavaboErrors>;
    async fn eliminar_lavabo(&self, id: Uuid) -> Result<(), LavaboErrors>;
    async fn obte_tots_lavabos(&self) -> Result<Vec<LavaboDTO>, LavaboErrors>;
    async fn obte_tots_lavabos_amb_etiquetes(&self) -> Result<Vec<LavaboAmbEtiquetesDTO>, LavaboErrors>;
    async fn obte_lavabo_detallat_per_id(&self, id: Uuid) -> Result<LavaboDetallatDTO, LavaboErrors>;
}

pub(crate) struct  LavaboServei {
    lavabo_repository : Arc<dyn LavaboRepository>,
    images_repository: Arc<dyn ImatgesRepository>,
    lavabo_imatges_repository: Arc<dyn LavaboImatgeRepository>,
    
}

impl LavaboServei {
    pub(crate) fn new(lavabo_repository: Arc<dyn LavaboRepository>, images_repository: Arc<dyn ImatgesRepository>, lavabo_imatges_repository: Arc<dyn LavaboImatgeRepository>) -> Self {
        Self { lavabo_repository, images_repository, lavabo_imatges_repository }
    }

}

#[async_trait]
impl LavaboService for LavaboServei {
    async fn crear_lavabo(&self ,lavabo_dto: LavaboDTO, imatges: Vec<FieldData<NamedTempFile>>, auth_data: AuthDataDTO, etiquetes: Vec<Uuid>) -> Result<(), LavaboErrors>{
        let lavabo_id = lavabo_dto.id.clone();
        self.lavabo_repository.crear_lavabo(lavabo_dto.into()).await?;

        for etiqueta in etiquetes {
            self.lavabo_repository.afegir_etiquetes_lavabo(etiqueta, lavabo_id.clone()).await?;
        }

        let mut errors: Vec<StorageError> = Vec::new();
        println!("{}", imatges.len());
        for imatge in imatges {
            let storage_imatge = StorageImatge{bucket_id: "lavabo_images".to_string(), directories_path: Some(format!("{}/", lavabo_id)), imatge};
            let imatge_lavabo = LavaboImatge::new(storage_imatge.imatge.metadata.file_name.as_deref().unwrap().to_string(), lavabo_id.clone(), Utc::now());
            let error = self.images_repository.pujar_imatge(&auth_data, storage_imatge).await;
            match error {
                Ok(_) => {
                    self.lavabo_imatges_repository.crear_imate_lavabo(imatge_lavabo).await?;
                },
                Err(err) => errors.push(err)
            }
        }
        if errors.len() > 0 {
            match &errors[0] {
                StorageError::ServerError(message) => Err(LavaboErrors::ImatgeNoPujada(message.clone())),
            }
            
        } else {
            Ok(())
        }
    }


    async fn obte_lavabo_per_id(&self, id: Uuid) -> Result<LavaboDTO, LavaboErrors>{
        let result = self.lavabo_repository.obte_lavabo_per_id(id).await;

        match result {
            Ok(lavabo) => {
                Ok(lavabo.into())
            },
            Err(error) => Err(error)
        }
    }

    async fn actualitzar_lavabo(&self, id: Uuid, lavabo_dto: LavaboDetallatDTO) -> Result<LavaboDetallatDTO, LavaboErrors>{
        
        let lavabo_antic= self.lavabo_repository.obte_lavabo_detallat_per_id(id.clone()).await?;
        let nou_lavabo : LavaboDetallat = lavabo_dto.into();
        
        let (etiquetes_afegir, etiquetes_eliminar) = calcular_diferencias_etiquetas(&lavabo_antic.etiquetes, &nou_lavabo.etiquetes);

        for etiqueta in etiquetes_afegir {
            self.lavabo_repository.afegir_etiquetes_lavabo(etiqueta, id.clone()).await?;
        }
        for etiqueta in etiquetes_eliminar {
            self.lavabo_repository.eliminar_etiqueta_lavabo(etiqueta, id.clone()).await?;
        }
        let lavabo_model = Lavabo{
            id: id.clone(),
            descripcio: nou_lavabo.descripcio,
            titol: nou_lavabo.titol,
            puntuacio_mitja: nou_lavabo.puntuacio_mitja,
            nombre_resenyes: nou_lavabo.nombre_resenyes,
            created_at: nou_lavabo.created_at,
            creador_id: nou_lavabo.creador_id,
            localitzacio: nou_lavabo.localitzacio
        };
        
        let result = self.lavabo_repository.actualitzar_lavabo(id.clone(), lavabo_model).await;
        
        let lav= self.lavabo_repository.obte_lavabo_detallat_per_id(id.clone()).await?;
        match result {
            Ok(()) => {
                let lavabo = self.lavabo_repository.obte_lavabo_per_id(id).await;
                match lavabo {
                    Ok(lvb) => Ok(lav.into()),
                    Err(error) => Err(error)
                }
            },
            Err(error) => Err(error)
        }
    }
    async fn eliminar_lavabo(&self, id: Uuid) -> Result<(), LavaboErrors>{
        self.lavabo_repository.eliminar_lavabo(id).await
    }

    async fn obte_tots_lavabos(&self) -> Result<Vec<LavaboDTO>, LavaboErrors> {
        let lavabos = self.lavabo_repository.obte_tots_lavabos().await?;
        Ok(lavabos.into_iter().map(Into::into).collect())
    }
    async fn obte_tots_lavabos_amb_etiquetes(&self) -> Result<Vec<LavaboAmbEtiquetesDTO>, LavaboErrors>{
        let lavabos_amb_etiquetes = self.lavabo_repository.obte_tots_lavabos_amb_etiquetes().await?;
        Ok(lavabos_amb_etiquetes.into_iter().map(Into::into).collect())        
    }

    async fn obte_lavabo_detallat_per_id(&self, id: Uuid) -> Result<LavaboDetallatDTO, LavaboErrors> {
        let lavabo = self.lavabo_repository.obte_lavabo_detallat_per_id(id).await?;
        Ok(lavabo.into())
    }
    
}
pub fn calcular_diferencias_etiquetas(
    etiquetas_antiguas: &Vec<Etiqueta>, 
    etiquetas_nuevas: &Vec<Etiqueta>
) -> (Vec<Uuid>, Vec<Uuid>) {
 
    let ids_antiguos: HashSet<Uuid> = etiquetas_antiguas.iter().map(|e| e.id).collect();
    let ids_nuevos: HashSet<Uuid> = etiquetas_nuevas.iter().map(|e| e.id).collect();

    let ids_a_eliminar: Vec<Uuid> = ids_antiguos.difference(&ids_nuevos).copied().collect();

    let ids_a_anyadir: Vec<Uuid> = ids_nuevos.difference(&ids_antiguos).copied().collect();

    (ids_a_anyadir,ids_a_eliminar)
}
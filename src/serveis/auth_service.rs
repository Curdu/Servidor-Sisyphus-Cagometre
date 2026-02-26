use std::{sync::Arc, time::{SystemTime, UNIX_EPOCH}};

use async_trait::async_trait;
use jsonwebtoken::{EncodingKey, Header, encode};
use sha2::{Digest, Sha512};

use crate::{dades::{models::usuari::Usuari, repositoris::user_repository::UserRepository}, errors::{auth_errors::AuthError, usuari_errors::UsuariErrors}, routes::extractors::auth_extractors::ClaimsInfo, state::SECRET_KEY};

use super::{dtos::{auth_dto::AuthToken, usuari_dto::UsuariDTO}, user_service::generar_hash};


#[async_trait]
pub(crate) trait AuthService: Send + Sync {
    async fn login(&self, correu: String, passwd: String) -> Result<AuthToken, AuthError>;
    async fn registre(&self, usuari : UsuariDTO) -> Result<(), UsuariErrors>;
}

pub(crate) struct AuthServei {
    usuari_repository : Arc<dyn UserRepository>
}

impl AuthServei {
    pub(crate) fn new(usuari_repository: Arc<dyn UserRepository>) -> Self {
        Self { usuari_repository }
    }
}

#[async_trait]
impl AuthService for AuthServei {

    async fn login(&self, correu: String, passwd: String) -> Result<AuthToken, AuthError>{
        let resposta = self.usuari_repository.obte_usuari_per_correu(correu).await;
        
        match resposta {
            Ok(usuari) =>{

                let mut hasher = Sha512::new();

                hasher.update(usuari.salt.clone().as_bytes());
                hasher.update(passwd.as_bytes());

                let hash = hasher.finalize();

                let hash_hex = hex::encode(hash);

                if usuari.hash != hash_hex {
                   return Err(AuthError::PasswdIncorrecte("La contrasenya es incorrecta".to_string()));
                }

                let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Algo ha fallat").as_secs() as u64;
                let expiracio = now + 3600;
                let claims = ClaimsInfo{user_id: usuari.id, exp: expiracio, correu: usuari.correu};
                let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET_KEY.clone().as_bytes()));
                match token {
                    Ok(token) => Ok(AuthToken::new(token)),
                    Err(error) => Err(AuthError::ServerError(error.to_string()))
                }
            },
            Err(error) => {
                Err(AuthError::UsuariError(error))
            }
            
        }

    }
    async fn registre(&self, usuari_dto : UsuariDTO) -> Result<(), UsuariErrors>{
        let (hash, salt) = generar_hash(&usuari_dto.contrasenya);
        let usuari = Usuari::new(usuari_dto.id, usuari_dto.correu, usuari_dto.nom , usuari_dto.cognoms, hash, salt, usuari_dto.created_at, usuari_dto.rol);
        self.usuari_repository.crear_usuari(usuari).await
    }
}
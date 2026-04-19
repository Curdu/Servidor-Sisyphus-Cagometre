use axum_typed_multipart::FieldData;
use tempfile::NamedTempFile;

pub(crate) struct StorageImatge {
    pub(crate) bucket_id : String,
    pub(crate) directories_path: Option<String>,
    pub(crate) imatge: FieldData<NamedTempFile>

}
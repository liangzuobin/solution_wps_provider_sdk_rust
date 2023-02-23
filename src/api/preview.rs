use crate::service::perview::Preview;
use crate::{
    error::provider_error::ProviderError,
    model::{file::File, response::ProviderResponse},
};
use actix_web::{get, web::Data, web::Json, web::Path};

#[get("/v3/3rd/files/{file_id}")]
async fn fetch_file(
    fs: Data<dyn Preview>,
    file_id: Path<String>,
) -> Result<Json<ProviderResponse<File>>, ProviderError> {
    match fs.fetch_file(file_id.into_inner()){
        Ok(file) => Ok(Json(ProviderResponse{
            code : 0,
            message: "ok".to_string(),
            data: file})),
        Err(err)  => Err(err),
    }
}

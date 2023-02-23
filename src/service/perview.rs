use crate::error::provider_error::ProviderError;
use crate::model::{download_info::DownloadInfo, file::File, permission::Permission};

pub trait Preview {
    fn fetch_file(&self, file_id: String) -> Result<File, ProviderError>;
    fn fetch_download_info( &self, file_id: String) -> Result<DownloadInfo, ProviderError>;
    fn fetch_user_permission(&self, file_id: String) -> Result<Permission, ProviderError>;
}

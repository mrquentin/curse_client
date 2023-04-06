use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::models::enums::{FileRelationType, FileReleaseType, FileStatus, HashAlgo, ModLoaderType};
use crate::models::games::SortableGameVersion;
use crate::models::pagination::Pagination;

#[derive(Deserialize)]
pub struct GetModFileResponse {
    pub data: File,
}

#[derive(Deserialize)]
pub struct GetModFilesResponse {
    pub data: Vec<File>,
    pub pagination: Pagination,
}

#[derive(Deserialize)]
pub struct GetFilesResponse {
    pub data: Vec<File>,
}

#[derive(Deserialize)]
pub struct FileHash {
    pub value: String,
    pub algo: HashAlgo,
}

#[derive(Deserialize)]
pub struct FileIndex {
    #[serde(rename = "gameVersion")]
    pub game_version: String,
    #[serde(rename = "fileId")]
    pub file_id: i32,
    pub filename: String,
    #[serde(rename = "releaseType")]
    pub release_type: FileReleaseType,
    #[serde(rename = "gameVersionTypeId")]
    pub game_version_type_id: Option<i32>,
    #[serde(rename = "modLoader")]
    pub mod_loader: ModLoaderType,
}

#[derive(Deserialize)]
pub struct FileDependency {
    #[serde(rename = "modId")]
    pub mod_id: i32,
    #[serde(rename = "relationType")]
    pub relation_type: FileRelationType,
}

#[derive(Deserialize)]
pub struct FileModule {
    pub name: String,
    pub fingerprint: i64,
}

#[derive(Deserialize)]
pub struct File {
    pub id: i32,
    #[serde(rename = "gameId")]
    pub game_id: i32,
    #[serde(rename = "modId")]
    pub mod_id: i32,
    #[serde(rename = "isAvailable")]
    pub is_available: bool,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "fileName")]
    pub file_name: String,
    #[serde(rename = "fileReleaseType")]
    pub release_type: FileReleaseType,
    #[serde(rename = "fileStatus")]
    pub file_status: FileStatus,
    pub hashes: Vec<FileHash>,
    #[serde(rename = "fileDate")]
    pub file_date: String,
    #[serde(rename = "fileLength")]
    pub file_length: i64,
    #[serde(rename = "downloadCount")]
    pub download_count: i64,
    #[serde(rename = "downloadUrl")]
    pub download_url: String,
    #[serde(rename = "gameVersions")]
    pub game_versions: Vec<String>,
    #[serde(rename = "sortableGameVersion")]
    pub sortable_game_version: Vec<SortableGameVersion>,
    pub dependencies: Vec<FileDependency>,
    #[serde(rename = "exposeAsAlternative")]
    pub expose_as_alternative: Option<bool>,
    #[serde(rename = "parentProjectFileId")]
    pub parent_project_file_id: Option<i32>,
    #[serde(rename = "alternateFileId")]
    pub alternate_file_id: Option<i32>,
    #[serde(rename = "isServerPack")]
    pub is_server_pack: Option<bool>,
    #[serde(rename = "serverPackFileId")]
    pub server_pack_file_id: Option<i32>,
    #[serde(rename = "isEarlyAccessContent")]
    pub is_early_access_content: Option<bool>,
    #[serde(rename = "earlyAccessEndDate")]
    pub early_access_end_date: Option<String>,
    #[serde(rename = "fileFingerprint")]
    pub file_fingerprint: i64,
    pub modules: Vec<FileModule>,
}

#[derive(Serialize)]
pub struct GetModFilesRequestBody {
    #[serde(rename = "fileIds")]
    pub file_ids: Vec<i32>,
}

impl GetModFilesRequestBody {
    pub fn new(file_ids: Vec<i32>) -> Self {
        Self { file_ids }
    }
}

#[derive(Serialize)]
pub struct GetFilesSearchParameters {
    #[serde(rename = "gameVersion")]
    pub game_version: Option<String>,
    #[serde(rename = "modLoaderType")]
    pub mod_loader_type: Option<ModLoaderType>,
    #[serde(rename = "gameVersionTypeId")]
    pub game_version_type_id: Option<i32>,
    pub index: Option<i32>,
    #[serde(rename = "pageSize")]
    pub page_size: Option<i32>,
}

impl GetFilesSearchParameters {
    pub fn to_query(&self) -> HashMap<&str, String> {
        let mut query = HashMap::new();

        if let Some(game_version) = &self.game_version { query.insert("gameVersion", game_version.to_string()); }
        if let Some(mod_loader_type) = &self.mod_loader_type { query.insert("modLoaderType", mod_loader_type.enum_value().to_string()); }
        if let Some(game_version_type_id) = &self.game_version_type_id { query.insert("gameVersionTypeId", game_version_type_id.to_string()); }
        if let Some(index) = &self.index { query.insert("index", index.to_string()); }
        if let Some(page_size) = &self.page_size { query.insert("pageSize", page_size.to_string()); }

        query
    }
}

impl Default for GetFilesSearchParameters {
    fn default() -> Self {
        Self {
            game_version: None,
            mod_loader_type: None,
            game_version_type_id: None,
            index: None,
            page_size: None,
        }
    }
}
use serde::Deserialize;
use crate::models::enums::{GameVersionStatus, ModLoaderInstallMethod, ModLoaderType};

#[derive(Deserialize)]
pub struct GetMinecraftGameVersionsResponse {
    pub data: Vec<MinecraftGameVersion>
}

#[derive(Deserialize)]
pub struct GetMinecraftGameVersionResponse {
    pub data: MinecraftGameVersion
}

#[derive(Deserialize)]
pub struct GetMinecraftModLoaderVersionResponse {
    pub data: MinecraftModLoaderVersion
}

#[derive(Deserialize)]
pub struct GetMinecraftModLoadersVersionResponse {
    pub data: Vec<MinecraftModLoaderIndex>
}

#[derive(Deserialize)]
pub struct MinecraftGameVersion {
    pub id: i32,
    #[serde(rename = "gameVersionId")]
    pub game_version_id: i32,
    #[serde(rename = "versionString")]
    pub version_string: String,
    #[serde(rename = "jarDownloadUrl")]
    pub jar_download_url: String,
    #[serde(rename = "jsonDownloadUrl")]
    pub json_download_url: String,
    pub approved: bool,
    #[serde(rename = "gameVersionTypeId")]
    pub game_version_type_id: i32,
    #[serde(rename = "gameVersionStatus")]
    pub game_version_status: GameVersionStatus,
    #[serde(rename = "gameVersionTypeStatus")]
    pub game_version_type_status: GameVersionTypeStatus,
}

#[derive(Deserialize)]
pub struct GameVersionTypeStatus {
    pub id: i32,
    #[serde(rename = "gameId")]
    pub game_id: i32,
    pub name: String,
    pub slug: String,
}

#[derive(Deserialize)]
pub struct MinecraftModLoaderVersion {
    pub id: i32,
    #[serde(rename = "gameVersionId")]
    pub game_version_id: i32,
    #[serde(rename = "minecraftGameVersionId")]
    pub minecraft_game_version_id: i32,
    #[serde(rename = "forgeVersion")]
    pub forge_version: String,
    pub name: String,
    #[serde(rename = "type")]
    pub loader_type: ModLoaderType,
    #[serde(rename = "downloadUrl")]
    pub download_url: String,
    pub filename: String,
    #[serde(rename = "installMethod")]
    pub install_method: ModLoaderInstallMethod,
    pub latest: bool,
    pub recommended: bool,
    pub approved: bool,
    #[serde(rename = "dateModified")]
    pub date_modified: String,
    #[serde(rename = "mavenVersionString")]
    pub maven_version_string: String,
    #[serde(rename = "versionJson")]
    pub version_json: String,
    #[serde(rename = "librariesInstallLocation")]
    pub libraries_install_location: String,
    #[serde(rename = "minecraftVersion")]
    pub minecraft_version: String,
    #[serde(rename = "additionalFilesJson")]
    pub additional_files_json: String,
    #[serde(rename = "modLoaderGameVersionId")]
    pub mod_loader_game_version_id: i32,
    #[serde(rename = "modLoaderGameVersionTypeId")]
    pub mod_loader_game_version_type_id: i32,
    #[serde(rename = "modLoaderGameVersionStatus")]
    pub mod_loader_game_version_status: GameVersionStatus,
    #[serde(rename = "modLoaderGameVersionTypeStatus")]
    pub mod_loader_game_version_type_status: GameVersionTypeStatus,
    #[serde(rename = "mcGameVersionId")]
    pub mc_game_version_id: i32,
    #[serde(rename = "mcGameVersionTypeId")]
    pub mc_game_version_type_id: i32,
    #[serde(rename = "mcGameVersionStatus")]
    pub mc_game_version_status: GameVersionStatus,
    #[serde(rename = "mcGameVersionTypeStatus")]
    pub mc_game_version_type_status: GameVersionTypeStatus,
    #[serde(rename = "installProfileJson")]
    pub install_profile_json: String,
}

#[derive(Deserialize)]
pub struct MinecraftModLoaderIndex {
    pub name: String,
    #[serde(rename = "gameVersion")]
    pub game_version: String,
    pub latest: bool,
    pub recommended: bool,
    #[serde(rename = "dateModified")]
    pub date_modified: String,
    #[serde(rename = "type")]
    pub loader_type: ModLoaderType,
}
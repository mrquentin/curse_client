use serde::Deserialize;
use crate::models::enums::{CoreApiStatus, CoreStatus};

#[derive(Deserialize, Debug)]
pub struct Game {
    pub id: i32,
    pub name: String,
    pub slug: String,
    #[serde(alias = "dateModified")]
    pub date_modified: String,
    pub assets: GameAsset,
    pub status: CoreStatus,
    #[serde(rename = "apiStatus")]
    pub api_status: CoreApiStatus,
}

#[derive(Deserialize, Debug)]
pub struct GameAsset {
    #[serde(rename = "iconUrl")]
    pub icon_url: Option<String>,
    #[serde(rename = "tileUrl")]
    pub tile_url: Option<String>,
    #[serde(rename = "coverUrl")]
    pub cover_url: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct GameVersion {
    pub id: i32,
    pub slug: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct SortableGameVersion {
    #[serde(alias = "gameVersionName")]
    pub game_version_name: String,
    #[serde(alias = "gameVersionPadded")]
    pub game_version_padded: String,
    #[serde(alias = "gameVersion")]
    pub game_version: String,
    #[serde(alias = "gameVersionReleaseDate")]
    pub game_version_release_date: String,
    #[serde(alias = "gameVersionTypeId")]
    pub game_version_type_id: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct GameVersionByType {
    #[serde(alias = "type")]
    pub version_type: i32,
    pub versions: Vec<GameVersion>,
}

#[derive(Deserialize, Debug)]
pub struct GameVersionType {
    pub id: i32,
    #[serde(alias = "gameId")]
    pub game_id: i32,
    pub name: String,
    pub slug: String,
}

#[derive(Deserialize, Debug)]
pub struct GetGamesResponse {
    pub data: Vec<Game>,
    pub pagination: super::pagination::Pagination,
}

#[derive(Deserialize, Debug)]
pub struct GetGameResponse {
    pub data: Game
}

#[derive(Deserialize, Debug)]
pub struct GetGameVersionsResponse {
    pub data: Vec<GameVersionByType>
}

#[derive(Deserialize, Debug)]
pub struct GetGameVersionTypesResponse {
    pub data: Vec<GameVersionType>
}
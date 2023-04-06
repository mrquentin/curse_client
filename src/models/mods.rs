use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::models::categories::Category;

use crate::models::enums::{ModLoaderType, ModsSearchSortField, ModStatus, SortOrder};
use crate::models::files::{File, FileIndex};
use crate::models::pagination::Pagination;

#[derive(Deserialize)]
pub struct SearchModsResponse {
    pub data: Vec<Mod>,
    pub pagination: Pagination,
}

#[derive(Deserialize)]
pub struct GetModResponse {
    pub data: Mod,
}

#[derive(Deserialize)]
pub struct GetModsResponse {
    pub data: Vec<Mod>,
}

#[derive(Deserialize)]
pub struct GetFeaturedModsResponse {
    pub data: FeaturedModsResponse,
}

#[derive(Serialize)]
pub struct GetModsByIdsListRequestBody {
    #[serde(rename = "modIds")]
    pub ids: Vec<i32>,
}

impl GetModsByIdsListRequestBody {
    pub fn new(ids: Vec<i32>) -> Self {
        Self { ids }
    }
}

#[derive(Serialize)]
pub struct GetFeaturedModsRequestBody {
    #[serde(rename = "gameId")]
    pub game_id: i32,
    #[serde(rename = "excludeModIds")]
    pub exclude_mod_ids: Vec<i32>,
    #[serde(rename = "gameVersionTypeId")]
    pub game_version_type_id: Option<i32>,
}

impl GetFeaturedModsRequestBody {
    pub fn new(game_id: i32, exclude_mod_ids: Vec<i32>, game_version_type_id: Option<i32>) -> Self {
        Self {
            game_id,
            exclude_mod_ids,
            game_version_type_id,
        }
    }
}

#[derive(Deserialize)]
pub struct ModLinks {
    #[serde(rename = "websiteUrl")]
    pub website_url: String,
    #[serde(rename = "wikiUrl")]
    pub wiki_url: String,
    #[serde(rename = "issuesUrl")]
    pub issues_url: String,
    #[serde(rename = "sourceUrl")]
    pub source_url: String,
}

#[derive(Deserialize)]
pub struct ModAuthor {
    pub id : i32,
    #[serde(rename = "modId")]
    pub mod_id : i32,
    pub title : String,
    pub description : String,
    #[serde(rename = "thumbnailUrl")]
    pub thumbnail_url : String,
    pub url : String,
}

#[derive(Deserialize)]
pub struct ModAsset {
    pub id : i32,
    #[serde(rename = "modId")]
    pub mod_id : i32,
    pub title : String,
    pub description : String,
    #[serde(rename = "thumbnailUrl")]
    pub thumbnail_url : String,
    pub url : String,
}

#[derive(Deserialize)]
pub struct Mod {
    pub id: i32,
    #[serde(alias = "gameId")]
    pub game_id: i32,
    pub name: String,
    pub slug: String,
    pub links: ModLinks,
    pub summary: String,
    pub status: ModStatus,
    #[serde(alias = "downloadCount")]
    pub download_count: i64,
    #[serde(alias = "isFeatured")]
    pub is_featured: bool,
    #[serde(alias = "primaryCategoryId")]
    pub primary_category_id: i32,
    pub categories: Vec<Category>,
    #[serde(alias = "classId")]
    pub class_id: Option<i32>,
    pub authors: Vec<ModAuthor>,
    pub logo: ModAsset,
    pub screenshots: Vec<ModAsset>,
    #[serde(alias = "mainFileId")]
    pub main_file_id: i32,
    #[serde(alias = "latestFiles")]
    pub latest_files: Vec<File>,
    #[serde(alias = "latestFilesIndexes")]
    pub latest_files_indexes: Vec<FileIndex>,
    #[serde(alias = "dateCreated")]
    pub date_created: String,
    #[serde(alias = "dateModified")]
    pub date_modified: String,
    #[serde(alias = "dateReleased")]
    pub date_released: String,
    #[serde(alias = "allowModDistribution")]
    pub allow_mod_distribution: Option<bool>,
    #[serde(alias = "gamePopularityRank")]
    pub game_popularity_rank: i32,
    #[serde(alias = "isAvailable")]
    pub is_available: bool,
    #[serde(alias = "thumbsUpCount")]
    pub thumbs_up_count: i32,
}

#[derive(Deserialize)]
pub struct FeaturedModsResponse {
    pub featured: Vec<Mod>,
    pub popular: Vec<Mod>,
    #[serde(alias = "recentlyUpdated")]
    pub recently_updated: Vec<Mod>,
}

pub struct ModSearchParameters {
    pub game_id: Option<i32>,
    pub class_id: Option<i32>,
    pub category_id: Option<i32>,
    pub game_version: Option<String>,
    pub search_filter: Option<String>,
    pub sort_field: Option<ModsSearchSortField>,
    pub sort_order: Option<SortOrder>,
    pub mod_loader_type: Option<ModLoaderType>,
    pub game_version_type_id: Option<i32>,
    pub author_id: Option<i32>,
    pub slug: Option<String>,
    pub index: Option<i32>,
    pub page_size: Option<i32>,
}

impl ModSearchParameters {
    pub fn new() -> ModSearchParameters {
        ModSearchParameters {
            game_id: None,
            class_id: None,
            category_id: None,
            game_version: None,
            search_filter: None,
            sort_field: None,
            sort_order: None,
            mod_loader_type: None,
            game_version_type_id: None,
            author_id: None,
            slug: None,
            index: None,
            page_size: None,
        }
    }

    pub fn to_query(&self) -> HashMap<&str, String> {
        let mut params = HashMap::new();

        if let Some(value) = self.game_id { params.insert("gameId", value.to_string()); }
        if let Some(value) = self.class_id { params.insert("classId", value.to_string()); }
        if let Some(value) = self.category_id { params.insert("categoryId", value.to_string()); }
        if let Some(value) = &self.game_version { params.insert("gameVersion", value.to_string()); }
        if let Some(value) = &self.search_filter { params.insert("searchFilter", value.to_string()); }
        if let Some(value) = &self.sort_field { params.insert("sortField", value.enum_value().to_string()); }
        if let Some(value) = &self.sort_order { params.insert("sortOrder", value.enum_value().to_string()); }
        if let Some(value) = &self.mod_loader_type { params.insert("modLoaderType", value.enum_value().to_string()); }
        if let Some(value) = self.game_version_type_id { params.insert("gameVersionTypeId", value.to_string()); }
        if let Some(value) = self.author_id { params.insert("authorId", value.to_string()); }
        if let Some(value) = &self.slug { params.insert("slug", value.to_string()); }
        if let Some(value) = self.index { params.insert("index", value.to_string()); }
        if let Some(value) = self.page_size { params.insert("pageSize", value.to_string()); }

        params
    }
}
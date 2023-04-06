use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::models::files::File;

#[derive(Serialize)]
pub struct GetFingerprintMatchesRequestBody {
    pub fingerprints: Vec<i32>,
}

impl GetFingerprintMatchesRequestBody {
    pub fn new(fingerprints: Vec<i32>) -> Self {
        Self { fingerprints }
    }
}

#[derive(Serialize)]
pub struct GetFuzzyMatchesRequestBody {
    #[serde(rename = "gameId")]
    pub game_id: i32,
    pub fingerprints: Vec<FolderFingerprint>,
}

impl GetFuzzyMatchesRequestBody {
    pub fn new(game_id: i32, folders: HashMap<String, Vec<i32>>) -> Self {
        Self { game_id, fingerprints: folders.into_iter().map(|(k, v)| FolderFingerprint::new(k, v)).collect() }
    }
}

#[derive(Serialize)]
pub struct FolderFingerprint {
    #[serde(rename = "folderName")]
    pub folder_name: String,
    pub fingerprints: Vec<i32>,
}

impl FolderFingerprint {
    pub fn new(folder_name: String, fingerprints: Vec<i32>) -> Self {
        Self { folder_name, fingerprints }
    }
}

#[derive(Deserialize)]
pub struct GetFingerprintMatchesResponse {
    pub data: FingerprintsMatchesResult,
}

#[derive(Deserialize)]
pub struct GetFingerprintFuzzyMatchesResponse {
    pub data: FingerprintFuzzyMatchResult,
}

#[derive(Deserialize)]
pub struct FingerprintMatch {
    pub id: i32,
    pub file: File,
    #[serde(rename = "latestFiles")]
    pub latest_files: Vec<File>,
}

#[derive(Deserialize)]
pub struct FingerprintsMatchesResult {
    #[serde(rename = "isCacheBuilt")]
    pub is_cache_built: bool,
    #[serde(rename = "exactMatches")]
    pub exact_matches: Vec<FingerprintMatch>,
    #[serde(rename = "exactFingerprints")]
    pub exact_fingerprints: Vec<i32>,
    #[serde(rename = "partialMatches")]
    pub partial_matches: Vec<FingerprintMatch>,
    #[serde(rename = "partialMatchFingerprints")]
    pub partial_match_fingerprints: HashMap<String, Vec<i32>>,
    #[serde(rename = "installedFingerprints")]
    pub installed_fingerprints: Vec<i32>,
    #[serde(rename = "unmatchedFingerprints")]
    pub unmatched_fingerprints: Vec<i32>,
}

#[derive(Deserialize)]
pub struct FingerprintFuzzyMatchResult {
    #[serde(rename = "fuzzyMatches")]
    pub fuzzy_matches: Vec<FingerprintFuzzyMatch>,
}

#[derive(Deserialize)]
pub struct FingerprintFuzzyMatch {
    pub id: i32,
    pub file: File,
    #[serde(rename = "latestFiles")]
    pub latest_files: Vec<File>,
    pub fingerprints: Vec<i32>,
}
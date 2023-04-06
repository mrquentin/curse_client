use std::collections::HashMap;
use reqwest::Url;
use anyhow::Result;
use crate::Client;
use crate::models::fingerprints::{FingerprintFuzzyMatchResult, FingerprintsMatchesResult, GetFingerprintFuzzyMatchesResponse, GetFingerprintMatchesRequestBody, GetFingerprintMatchesResponse, GetFuzzyMatchesRequestBody};

pub struct FingerprintsEndpoint<'c>(pub &'c Client);

impl<'c> FingerprintsEndpoint<'c> {
    fn endpoint(&self) -> Result<Url> {
        Ok(self.0.api_base.join("/v1/fingerprints/")?)
    }

    pub async fn get_fingerprints_by_game_id(&self, game_id: i32, fingerprints: Vec<i32>) -> Result<FingerprintsMatchesResult> {
        let body = GetFingerprintMatchesRequestBody::new(fingerprints);
        let endpoint = self.endpoint()?.join(game_id.to_string().as_str())?;
        let resp = self.0.client.post(endpoint).body(&body).send().await?;

        Ok(resp.json::<GetFingerprintMatchesResponse>().await?.data)
    }

    pub async fn get_fingerprints(&self, fingerprints: Vec<i32>) -> Result<FingerprintsMatchesResult> {
        let body = GetFingerprintMatchesRequestBody::new(fingerprints);
        let endpoint = self.endpoint()?;
        let resp = self.0.client.post(endpoint).body(&body).send().await?;

        Ok(resp.json::<GetFingerprintMatchesResponse>().await?.data)
    }

    pub async fn get_fingerprints_by_game_id_fuzzy(&self, game_id: i32, folders: HashMap<String, Vec<i32>>) -> Result<FingerprintFuzzyMatchResult> {
        let body = GetFuzzyMatchesRequestBody::new(game_id, folders);
        let endpoint = self.endpoint()?.join("/fuzzy")?.join(game_id.to_string().as_str())?;
        let resp = self.0.client.post(endpoint).body(&body).send().await?;

        Ok(resp.json::<GetFingerprintFuzzyMatchesResponse>().await?.data)
    }

    pub async fn get_fingerprints_fuzzy(&self, game_id: i32, folders: HashMap<String, Vec<i32>>) -> Result<FingerprintFuzzyMatchResult> {
        let body = GetFuzzyMatchesRequestBody::new(game_id, folders);
        let endpoint = self.endpoint()?.join("/fuzzy")?;
        let resp = self.0.client.post(endpoint).body(&body).send().await?;

        Ok(resp.json::<GetFingerprintFuzzyMatchesResponse>().await?.data)
    }
}
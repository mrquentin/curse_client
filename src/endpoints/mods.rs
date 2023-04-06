use reqwest::Url;
use anyhow::Result;
use crate::Client;
use crate::models::mods::{FeaturedModsResponse, GetFeaturedModsRequestBody, GetFeaturedModsResponse, GetModResponse, GetModsByIdsListRequestBody, GetModsResponse, Mod, ModSearchParameters, SearchModsResponse};

pub struct ModsEndpoint<'c>(pub &'c Client);

impl<'c> ModsEndpoint<'c> {
    fn endpoint(&self) -> Result<Url> {
        Ok(self.0.api_base.join("/v1/mods/")?)
    }

    pub async fn search(&self, params: ModSearchParameters) -> Result<Vec<Mod>> {
        let endpoint = self.endpoint()?.join("search")?;
        let resp = self.0.client.get(endpoint).query(&params.to_query()).send().await?;

        Ok(resp.json::<SearchModsResponse>().await?.data)
    }

    pub async fn get_mod(&self, mod_id: &str) -> Result<Mod> {
        let endpoint = self.endpoint()?.join(mod_id)?;
        let resp = self.0.client.get(endpoint).send().await?;

        Ok(resp.json::<GetModResponse>().await?.data)
    }

    pub async fn get_mods_by_ids(&self, mod_ids: Vec<i32>) -> Result<Vec<Mod>>{
        let body = GetModsByIdsListRequestBody::new(mod_ids);
        let endpoint = self.endpoint()?;
        let resp = self.0.client.post(endpoint).json(&body).send().await?;

        Ok(resp.json::<GetModsResponse>().await?.data)
    }

    pub async fn get_featured_mods(&self, game_id: i32, exclude_mod_ids: Vec<i32>, game_version_type_id: Option<i32>) -> Result<FeaturedModsResponse> {
        let body = GetFeaturedModsRequestBody::new(game_id, exclude_mod_ids, game_version_type_id);
        let endpoint = self.endpoint()?.join("featured")?;
        let resp = self.0.client.post(endpoint).json(&body).send().await?;

        Ok(resp.json::<GetFeaturedModsResponse>().await?.data)
    }

    pub async fn get_mod_description(&self, mod_id: &str) -> Result<String> {
        let endpoint = self.endpoint()?.join(mod_id)?.join("description")?;
        let resp = self.0.client.get(endpoint).send().await?;

        Ok(resp.text().await?)
    }
}
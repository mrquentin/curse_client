use reqwest::Url;
use anyhow::Result;
use crate::Client;

use crate::models::games::{Game, GameVersionByType, GameVersionType, GetGameResponse, GetGamesResponse, GetGameVersionsResponse, GetGameVersionTypesResponse};

pub struct GamesEndpoint<'c>(pub &'c Client);

impl<'c> GamesEndpoint<'c> {
    fn endpoint_v1(&self) -> Result<Url> {
        Ok(self.0.api_base.join("/v1/games")?)
    }

    fn endpoint_v2(&self) -> Result<Url> {
        Ok(self.0.api_base.join("/v2/games")?)
    }

    pub async fn get_games(&self) -> Result<Vec<Game>> {
        let endpoint = self.endpoint_v1()?;
        let resp = self.0.client.get(endpoint).send().await?;

        Ok(resp.json::<GetGamesResponse>().await?.data)
    }

    pub async fn get_game(&self, id: &str) -> Result<Game> {
        let endpoint = self.endpoint_v1()?.join(id)?;
        let resp = self.0.client.get(endpoint).send().await?;

        Ok(resp.json::<GetGameResponse>().await?.data)
    }

    pub async fn get_versions(&self, id: &str) -> Result<Vec<GameVersionByType>> {
        let endpoint = self.endpoint_v2()?.join(id)?.join("/versions")?;
        let resp = self.0.client.get(endpoint).send().await?;

        Ok(resp.json::<GetGameVersionsResponse>().await?.data)
    }

    pub async fn get_version_types(&self, id: &str) -> Result<Vec<GameVersionType>> {
        let endpoint = self.endpoint_v1()?.join(id)?.join("/version-types")?;
        let resp = self.0.client.get(endpoint).send().await?;

        Ok(resp.json::<GetGameVersionTypesResponse>().await?.data)
    }

}
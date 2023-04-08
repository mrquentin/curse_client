use reqwest::Url;
use anyhow::Result;
use crate::CurseClient;

use crate::models::games::{Game, GameVersionByType, GameVersionType, GetGameResponse, GetGamesResponse, GetGameVersionsResponse, GetGameVersionTypesResponse};

pub struct GamesEndpoint<'c>(pub &'c CurseClient);

impl<'c> GamesEndpoint<'c> {

    pub async fn get_games(&self) -> Result<Vec<Game>> {
        let endpoint = self.0.api_base.join("v1/games/")?;
        let resp = self.0.client.get(endpoint).send().await?;

        Ok(resp.json::<GetGamesResponse>().await?.data)
    }

    pub async fn get_game(&self, id: i32) -> Result<Game> {
        let endpoint = self.0.api_base.join(&format!("v1/games/{}", id))?;
        let resp = self.0.client.get(endpoint).send().await?;

        Ok(resp.json::<GetGameResponse>().await?.data)
    }

    pub async fn get_versions(&self, id: i32) -> Result<Vec<GameVersionByType>> {
        let endpoint = self.0.api_base.join(&format!("v2/games/{}/versions", id))?;
        let resp = self.0.client.get(endpoint).send().await?;

        Ok(resp.json::<GetGameVersionsResponse>().await?.data)
    }

    pub async fn get_version_types(&self, id: i32) -> Result<Vec<GameVersionType>> {
        let endpoint = self.0.api_base.join(&format!("v1/games/{}/version-types", id))?;
        let resp = self.0.client.get(endpoint).send().await?;

        Ok(resp.json::<GetGameVersionTypesResponse>().await?.data)
    }

}
use reqwest::Url;
use anyhow::Result;
use crate::CurseClient;

use crate::models::categories::{Category, GetCategoriesResponse};

pub struct CategoriesEndpoint<'c>(pub &'c CurseClient);

impl<'c> CategoriesEndpoint<'c> {
    fn endpoint(&self) -> Result<Url> {
        Ok(self.0.api_base.join("/v1/categories")?)
    }

    pub async fn get_categories_by_game(&self, game_id: &str, classes_only: bool) -> Result<Vec<Category>> {
        let endpoint = self.endpoint()?;
        let query = vec![("gameId", game_id.to_string()), ("classesOnly", classes_only.to_string())];
        let resp = self.0.client.get(endpoint).query(&query).send().await?;

        Ok(resp.json::<GetCategoriesResponse>().await?.data)
    }

    pub async fn get_categories_by_class(&self, class_id: &str) -> Result<Vec<Category>> {
        let endpoint = self.endpoint()?.join(class_id)?;
        let query = vec![("classId", class_id)];
        let resp = self.0.client.get(endpoint).query(&query).send().await?;

        Ok(resp.json::<GetCategoriesResponse>().await?.data)
    }
}
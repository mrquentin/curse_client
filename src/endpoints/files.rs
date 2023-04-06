use reqwest::Url;
use anyhow::Result;
use crate::Client;

use crate::models::files::{File, GetFilesResponse, GetFilesSearchParameters, GetModFileResponse, GetModFilesRequestBody, GetModFilesResponse};

pub struct FilesEndpoint<'c>(pub &'c Client);

impl<'c> FilesEndpoint<'c> {
    fn endpoint(&self) -> Result<Url> {
        Ok(self.0.api_base.join("/v1/mods/")?)
    }

    pub async fn get_mod_file(&self, mod_id: &str, file_id: &str) -> Result<File> {
        let endpoint = self.endpoint()?.join(mod_id)?.join("/files/").join(file_id)?;
        let resp = self.0.client.get(endpoint).send().await?;

        Ok(resp.json::<GetModFileResponse>().await?.data)
    }

    pub async fn get_mod_files(&self, mod_id: &str, search_params: Option<GetFilesSearchParameters>) -> Result<Vec<File>> {
        let query = search_params.unwrap_or_default().to_query();
        let endpoint = self.endpoint()?.join(mod_id)?.join("/files")?;
        let resp = self.0.client.get(endpoint).query(&query).send().await?;

        Ok(resp.json::<GetModFilesResponse>().await?.data)
    }

    pub async fn get_files(&self, file_ids: Vec<i32>) -> Result<Vec<File>> {
        let body = GetModFilesRequestBody::new(file_ids);
        let endpoint = self.endpoint()?.join("/files")?;
        let resp = self.0.client.post(endpoint).body(&body).send().await?;

        Ok(resp.json::<GetFilesResponse>().await?.data)
    }

    pub async fn get_mod_file_changelog(&self, mod_id: &str, file_id: &str) -> Result<String> {
        let endpoint = self.endpoint()?.join(mod_id)?.join("/files/").join(file_id)?.join("/changelog")?;
        let resp = self.0.client.get(endpoint).send().await?;

        Ok(resp.text().await?)
    }

    pub async fn get_mod_file_download_url(&self, mod_id: &str, file_id: &str) -> Result<String> {
        let endpoint = self.endpoint()?.join(mod_id)?.join("/files/").join(file_id)?.join("/download-url")?;
        let resp = self.0.client.get(endpoint).send().await?;

        Ok(resp.text().await?)
    }
}
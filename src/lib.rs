mod endpoints;
mod models;

use anyhow::Result;
use reqwest::{Url, header::{HeaderMap, HeaderValue}};
use std::sync::Arc;

pub struct CurseClient {
    api_base: Url,
    client: Arc<reqwest::Client>,
}

impl CurseClient {
    pub fn new(key: String) -> Result<CurseClient> {
        let api_base = Url::parse("https://api.curseforge.com")?;

        let mut headers = HeaderMap::new();
        headers.insert("x-api-key", HeaderValue::from_str(key.as_str())?);

        let client = Arc::new(reqwest::Client::builder()
            .default_headers(headers)
            .build()?);

        Ok(CurseClient { api_base, client })
    }

    pub fn games(&self) -> endpoints::games::GamesEndpoint {
        endpoints::games::GamesEndpoint(self)
    }

    pub fn categories(&self) -> endpoints::categories::CategoriesEndpoint {
        endpoints::categories::CategoriesEndpoint(self)
    }

    pub fn mods(&self) -> endpoints::mods::ModsEndpoint {
        endpoints::mods::ModsEndpoint(self)
    }

    pub fn files(&self) -> endpoints::files::FilesEndpoint {
        endpoints::files::FilesEndpoint(self)
    }

    pub fn fingerprints(&self) -> endpoints::fingerprints::FingerprintsEndpoint {
        endpoints::fingerprints::FingerprintsEndpoint(self)
    }

    pub fn minecraft(&self) -> endpoints::minecraft::MinecraftEndpoint {
        endpoints::minecraft::MinecraftEndpoint(self)
    }
}
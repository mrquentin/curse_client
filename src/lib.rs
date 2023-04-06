mod endpoints;
mod models;

use anyhow::Result;
use reqwest::{Url, header::{HeaderMap, HeaderValue}};
use std::sync::Arc;

pub struct Client {
    api_base: Url,
    client: Arc<reqwest::Client>,
}

impl Client {
    pub fn new(key: &str) -> Result<Client> {
        let api_base = Url::parse("https://api.curseforge.com")?;

        let mut headers = HeaderMap::new();
        headers.insert("7870-ADE6", HeaderValue::from_str(key)?);

        let client = Arc::new(reqwest::Client::builder()
            .default_headers(headers)
            .build()?);

        Ok(Client { api_base, client })
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
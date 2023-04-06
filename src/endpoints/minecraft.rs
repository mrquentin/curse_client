use reqwest::Url;
use anyhow::Result;
use crate::Client;
use crate::models::minecraft::{GetMinecraftGameVersionResponse, GetMinecraftGameVersionsResponse, GetMinecraftModLoadersVersionResponse, GetMinecraftModLoaderVersionResponse, MinecraftGameVersion, MinecraftModLoaderIndex, MinecraftModLoaderVersion};

pub struct MinecraftEndpoint<'c>(pub &'c Client);

impl<'c> MinecraftEndpoint<'c> {
    fn endpoint(&self) -> Result<Url> {
        Ok(self.0.api_base.join("/v1/minecraft/")?)
    }

    pub async fn get_minecraft_versions(&self) -> Result<Vec<MinecraftGameVersion>> {
        let endpoint = self.endpoint()?.join("version")?;
        let resp = self.0.client.get(endpoint).send().await?;

        Ok(resp.json::<GetMinecraftGameVersionsResponse>().await?.data)
    }

    pub async fn get_minecraft_version(&self, version: &str) -> Result<MinecraftGameVersion> {
        let endpoint = self.endpoint()?.join("version").unwrap().join(version)?;
        let resp = self.0.client.get(endpoint).send().await?;

        Ok(resp.json::<GetMinecraftGameVersionResponse>().await?.data)
    }

    pub async fn get_minecraft_mod_loaders(&self) -> Result<Vec<MinecraftModLoaderIndex>> {
        let endpoint = self.endpoint()?.join("modloader")?;
        let resp = self.0.client.get(endpoint).send().await?;

        Ok(resp.json::<GetMinecraftModLoadersVersionResponse>().await?.data)
    }

    pub async fn get_minecraft_mod_loader(&self, mod_loader: &str) -> Result<MinecraftModLoaderVersion> {
        let endpoint = self.endpoint()?.join("modloader").unwrap().join(mod_loader)?;
        let resp = self.0.client.get(endpoint).send().await?;

        Ok(resp.json::<GetMinecraftModLoaderVersionResponse>().await?.data)
    }
}
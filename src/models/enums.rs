use serde::{Deserialize, Serialize};
use serde_repr::Deserialize_repr;

#[derive(Deserialize)]
pub enum ModLoaderInstallMethod {
    #[serde(rename = "1")]
    ForgeInstaller,
    #[serde(rename = "2")]
    ForgeJarInstall,
    #[serde(rename = "3")]
    ForgeInstallerV2,
}

#[derive(Deserialize, Serialize)]
pub enum ModLoaderType {
    #[serde(rename = "0")]
    Any,
    #[serde(rename = "1")]
    Forge,
    #[serde(rename = "2")]
    Cauldron,
    #[serde(rename = "3")]
    LiteLoader,
    #[serde(rename = "4")]
    Fabric,
    #[serde(rename = "5")]
    Quilt,
}

impl ModLoaderType {
    pub fn enum_value(&self) -> u8 {
        match self {
            ModLoaderType::Any => 0,
            ModLoaderType::Forge => 1,
            ModLoaderType::Cauldron => 2,
            ModLoaderType::LiteLoader => 3,
            ModLoaderType::Fabric => 4,
            ModLoaderType::Quilt => 5,
        }
    }
}

impl Default for ModLoaderType {
    fn default() -> Self {
        ModLoaderType::Any
    }
}

#[derive(Deserialize, Serialize)]
pub enum ModsSearchSortField {
    #[serde(rename = "1")]
    Featured,
    #[serde(rename = "2")]
    Popularity,
    #[serde(rename = "3")]
    LastUpdated,
    #[serde(rename = "4")]
    Name,
    #[serde(rename = "5")]
    Author,
    #[serde(rename = "6")]
    TotalDownloads,
    #[serde(rename = "7")]
    Category,
    #[serde(rename = "8")]
    GameVersion,
}

impl ModsSearchSortField {
    pub fn enum_value(&self) -> u8 {
        match self {
            ModsSearchSortField::Featured => 1,
            ModsSearchSortField::Popularity => 2,
            ModsSearchSortField::LastUpdated => 3,
            ModsSearchSortField::Name => 4,
            ModsSearchSortField::Author => 5,
            ModsSearchSortField::TotalDownloads => 6,
            ModsSearchSortField::Category => 7,
            ModsSearchSortField::GameVersion => 8,
        }
    }
}

impl Default for ModsSearchSortField {
    fn default() -> Self {
        ModsSearchSortField::Featured
    }
}

#[derive(Deserialize)]
pub enum ModStatus {
    #[serde(rename = "1")]
    New,
    #[serde(rename = "2")]
    ChangesRequired,
    #[serde(rename = "3")]
    UnderSoftReview,
    #[serde(rename = "4")]
    Approved,
    #[serde(rename = "5")]
    Rejected,
    #[serde(rename = "6")]
    ChangesMade,
    #[serde(rename = "7")]
    Inactive,
    #[serde(rename = "8")]
    Abandoned,
    #[serde(rename = "9")]
    Deleted,
    #[serde(rename = "10")]
    UnderReview,
}

#[derive(Deserialize, Serialize)]
pub enum SortOrder {
    #[serde(rename = "asc")]
    Ascending,
    #[serde(rename = "desc")]
    Descending,
}

impl SortOrder {
    pub fn enum_value(&self) -> &str {
        match self {
            SortOrder::Ascending => "asc",
            SortOrder::Descending => "desc",
        }
    }
}

impl Default for SortOrder {
    fn default() -> Self {
        SortOrder::Ascending
    }
}

#[derive(Deserialize)]
pub enum FileRelationType {
    #[serde(rename = "1")]
    EmbeddedLibrary,
    #[serde(rename = "2")]
    OptionalDependency,
    #[serde(rename = "3")]
    RequiredDependency,
    #[serde(rename = "4")]
    Tool,
    #[serde(rename = "5")]
    Incompatible,
    #[serde(rename = "6")]
    Include,
}

#[derive(Deserialize)]
pub enum FileReleaseType {
    #[serde(rename = "1")]
    Release,
    #[serde(rename = "2")]
    Beta,
    #[serde(rename = "3")]
    Alpha,
}

#[derive(Deserialize)]
pub enum FileStatus {
    #[serde(rename = "1")]
    Processing,
    #[serde(rename = "2")]
    ChangesRequired,
    #[serde(rename = "3")]
    UnderReview,
    #[serde(rename = "4")]
    Approved,
    #[serde(rename = "5")]
    Rejected,
    #[serde(rename = "6")]
    MalwareDetected,
    #[serde(rename = "7")]
    Deleted,
    #[serde(rename = "8")]
    Archived,
    #[serde(rename = "9")]
    Testing,
    #[serde(rename = "10")]
    Released,
    #[serde(rename = "11")]
    ReadyForReview,
    #[serde(rename = "12")]
    Deprecated,
    #[serde(rename = "13")]
    Baking,
    #[serde(rename = "14")]
    AwaitingPublishing,
    #[serde(rename = "15")]
    FailedPublishing,
}

#[derive(Deserialize)]
pub enum HashAlgo {
    #[serde(rename = "1")]
    SHA1,
    #[serde(rename = "2")]
    MD5,
}

#[derive(Deserialize)]
pub enum GameVersionStatus {
    #[serde(rename = "1")]
    Released,
    #[serde(rename = "2")]
    Beta,
    #[serde(rename = "3")]
    Alpha,
}


#[derive(Deserialize)]
pub enum GameVersionTypeStatus {
    #[serde(rename = "1")]
    Normal,
    #[serde(rename = "2")]
    Deleted,
}

#[derive(Deserialize_repr, Debug)]
#[repr(u8)]
pub enum CoreStatus {
    Draft = 1,
    Test = 2,
    PendingReview = 3,
    Rejected = 4,
    Approved = 5,
    Live = 6,
}

#[derive(Deserialize_repr, Debug)]
#[repr(u8)]
pub enum CoreApiStatus {
    Private = 1,
    Public = 2,
}
